#![cfg(fuzzing)]

mod main;

pub fn find_device_type<R>(reader: &mut R) -> Result<String, ()>
where
    R: std::io::Read + std::io::Seek,
{
    main::find_device_type(reader).map_err(drop)
}
