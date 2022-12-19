# Motivation

For some reason, I couldn’t ever feel excited or motivated enough about
“_Front-End_” side of web development to build some fully-fledged web,
cross-platform applications or to even improve my abilities and skill set about
the topic. Maybe it is because of JavaScript ecosystem or maybe JavaScript
itself. I don’t really know. There is something about it; sneaky and dirty that
makes me not want to touch it.

So how can I write a web application without writing a single line of Js? I
know, I know yeess… WebAssembly. Since I’m not some kind of a computer guru or
a tech influencer , I won’t even bother to explain what it is, how it is more
advantageous or disadvantageous against Js etc. One thing that is interesting
about WebAssembly and that I “can” explain or “quote” is the first sentence you 
read on WebAssembly wiki page 😎

WebAssembly (sometimes abbreviated Wasm) defines a portable binary-code format
and a corresponding text format for executable programs as well as software
interfaces for facilitating interactions between such programs and their host
environment.

You may noticed that there is no “Web” word in the definition (except the name
itself) 🤔. That is because WebAssembly is NOT specific to Web. In theory, you
should be able to execute your code on any structured stack machine with minimum
effort as long as it is compiled to a wasm target.

I’ve been really into Rust lately and there is something about it; sneaky and
dirty that makes me want not just to touch but absorb it through my skin.
And guess what? You CAN target your Rust code to be compiled to wasm 😈

This repository consists the code implementations that I referred in my Medium
story called "[Yet Another Yew App: Rust, wasm, Server Sent Events and more](https://medium.com/@itwasneo/yet-another-yew-app-rust-wasm-server-sent-events-and-more-5cd801d687b)"
Don't forget to give a clap 🤜🤛

---
Run the dummy-sse-services with
```bash
docker-compose --file docker-compose-dummy-sse-services.yml up -d --build
```

Run the Yew Application with
```
trunk serve
```
