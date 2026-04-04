# Introduccion.

> [!IMPORTANT]  
> At this point i'm still developing the project,so, it just accept BMP with no compresion.

This program reads a BMP image, decodes and parses it, and renders the result directly in the console. I built this project as a way to learn Rust while also gaining a deeper understanding of how binary data is read and interpreted.

I started by implementing a BMP parser because it is one of the simplest image formats to understand. A BMP file is structured in a straightforward way, consisting mainly of a file header, a DIB header, and a pixel array.

<img src="https://libsiowdujmvaygtfygf.supabase.co/storage/v1/object/sign/images/public/bitmap_format-10519205861437935660.png?token=eyJraWQiOiJzdG9yYWdlLXVybC1zaWduaW5nLWtleV85MzRlMGFiYi00Yzk2LTQ4OWMtOWJhMC1jMWRlZjlhMGQwZTIiLCJhbGciOiJIUzI1NiJ9.eyJ1cmwiOiJpbWFnZXMvcHVibGljL2JpdG1hcF9mb3JtYXQtMTA1MTkyMDU4NjE0Mzc5MzU2NjAucG5nIiwiaWF0IjoxNzc1MjczMDA1LCJleHAiOjE3NzUzNTk0MDV9.nKt-rqBMguBqQfKinu5RwVcVc8BJfE71Vlw8a1J1k_w">

> [!NOTE]  
> This image is from the youtuber [Kay Lack](https://www.youtube.com/@neoeno4242)

## How program works?

The project has 2 parts, the parsing lib and then the renderer.

In order to run the program you should run the following command:

```rs
cargo run PATH_TO_BMP 
```
There are a folder inside the project call "test_image" where you can find images for test it.

## Resources that helped me?

[Making .BMP images from scratch by Kay Lack](https://www.youtube.com/watch?v=13E0il2zxBA&t)

[BMP file format - Wikipedia](https://en.wikipedia.org/wiki/BMP_file_format)
