/*
 * Copyright (c) 2020-2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use mozjpeg_sys::*;
use std::convert::TryFrom;
use std::error::Error;
use std::ffi::CString;
use std::time::SystemTime;

use super::error::SimpleError;
use super::image::Image;

const JPEG_MARKER_MAX_LEN: usize = 0xFFFF;
const ICC_MARKER: u8 = mozjpeg_sys::jpeg_marker::APP0 as u8 + 2;

fn load_profile(
    cinfo: &mozjpeg_sys::jpeg_decompress_struct,
) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    const ICC_HEADER_LEN: usize = 14;

    let mut icc_markers = Vec::new();

    let mut marker_ptr = cinfo.marker_list;
    while !marker_ptr.is_null() {
        let marker = unsafe { &*marker_ptr };
        let data_len = usize::try_from(marker.data_length)?;
        if marker.marker == ICC_MARKER && data_len > ICC_HEADER_LEN {
            let data = unsafe { std::slice::from_raw_parts(marker.data, data_len) };
            if data[ICC_HEADER_LEN - 2] <= data[ICC_HEADER_LEN - 1]
                && b"ICC_PROFILE\0" == &data[0..ICC_HEADER_LEN - 2]
            {
                icc_markers.push(data);
            }
            marker_ptr = marker.next;
        }
    }

    if !icc_markers.is_empty() {
        let mut icc: Vec<u8> = Vec::new();
        icc_markers.sort_by_key(|data| data[ICC_HEADER_LEN - 2]);
        for data in icc_markers {
            icc.extend(&data[ICC_HEADER_LEN..]);
        }
        Ok(Some(icc))
    } else {
        Ok(None)
    }
}

pub fn load_jpeg(file: &str) -> Result<Image, Box<dyn Error>> {
    let start = SystemTime::now();

    let mut result: Image = Image::new();

    let path = std::path::Path::new(file);
    if !path.is_file() {
        return Err(SimpleError::to_box(&format!(
            "Not a file: {}",
            path.to_str().unwrap()
        )));
    }

    let mut err: jpeg_error_mgr = unsafe { std::mem::zeroed() };
    let mut cinfo: jpeg_decompress_struct = unsafe { std::mem::zeroed() };

    cinfo.common.err = unsafe { jpeg_std_error(&mut err) };
    unsafe { jpeg_create_decompress(&mut cinfo) };

    let file = CString::new(file.as_bytes()).unwrap();
    let mode = CString::new("rb").unwrap();
    let fh = unsafe { libc::fopen(file.as_ptr(), mode.as_ptr()) };
    unsafe { jpeg_stdio_src(&mut cinfo, fh) };
    unsafe { jpeg_save_markers(&mut cinfo, ICC_MARKER as c_int, JPEG_MARKER_MAX_LEN as u32) };
    unsafe { jpeg_read_header(&mut cinfo, true as boolean) };

    result.icc = load_profile(&cinfo)?;
    result.width = cinfo.image_width;
    result.height = cinfo.image_height;

    cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
    unsafe { jpeg_start_decompress(&mut cinfo) };

    result.pixel_components = u32::try_from(cinfo.output_components)?;
    let row_len = (result.width * result.pixel_components) as usize;
    let mut buffer = vec![0; result.height as usize * row_len];

    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * row_len;
        let mut jsamp_array = [buffer[offset..].as_mut_ptr()];
        unsafe { jpeg_read_scanlines(&mut cinfo, jsamp_array.as_mut_ptr(), 1) };
    }
    unsafe { jpeg_finish_decompress(&mut cinfo) };
    unsafe { jpeg_destroy_decompress(&mut cinfo) };
    unsafe { libc::fclose(fh) };

    result.data = buffer;

    apply_icc(&mut result);

    if cfg!(debug_assertions) {
        println!(
            "JPEG loaded in {} msec",
            SystemTime::now().duration_since(start).unwrap().as_millis()
        );
    }

    Ok(result)
}

pub fn apply_icc(img:&mut Image) {
    if img.icc.is_some() {
        let start = SystemTime::now();

        let icc = img.icc.as_mut().unwrap();
        use lcms2_sys::ffi::*;
        let profile_in =
            unsafe { cmsOpenProfileFromMem(icc.as_mut_ptr() as *mut c_void, icc.len() as u32) };
        let profile_out = unsafe { cmsCreate_sRGBProfile() };
        let transform = unsafe {
            cmsCreateTransform(
                profile_in,
                PixelFormat::RGB_8,
                profile_out,
                PixelFormat::RGB_8,
                Intent::Perceptual,
                0,
            )
        };
        unsafe { cmsCloseProfile(profile_out) };
        unsafe { cmsCloseProfile(profile_in) };

        unsafe {
            cmsDoTransform(
                transform,
                img.data.as_mut_ptr() as *mut c_void,
                img.data.as_mut_ptr() as *mut c_void,
                img.width * img.height,
            )
        };
        unsafe { cmsDeleteTransform(transform) };

        if cfg!(debug_assertions) {
            println!(
                "Applied color profile in {} msec",
                SystemTime::now().duration_since(start).unwrap().as_millis(),
            );
        }
    } else {
        if cfg!(debug_assertions) {
            println!("No color profile.");
        }
    }
}