# Multimedia Format Interface
Compress, decompress, encode, and decode in most formats.

## Code
```rust
extern crate mfi;

fn main() {
	// Encode APNG with 2 frames, 1/2 second
	let output = mfi::Png
		// Set video resolution
		.video(1u16, 1u16)
		// Add a frame
		.frame(&[0xFFu8, 0x00, 0x00])
		// Animation sleep for 0.5 seconds
		.sleep(0.5)
		// Add a frame
		.frame(&[0x00u8, 0x00, 0xFF])
		// Animation sleep for 0.5 seconds
		.sleep(0.5)
		// Return Vec of APNG animation bytes.
		.repeat();
	// Decode the APNG
	let mut decoder = mfi::Decoder::new(output.as_slice());
	assert_eq!(mfi::Png, decoder.format());
	assert_eq!((1u16, 1u16), decoder.wh());
	assert_eq!(
	
	let (w, h, data) = video.unwrap();
	assert_eq!(2, data.0.len());
	assert_eq!(3, data.0[0].len());
	assert_eq!(0.5, data.1);
}
```
