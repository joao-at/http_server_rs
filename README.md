# Introduction

An HTTP server written in rust. (Yes, another one, it's an amazing starting project to get comfortable with any programming language)

This is a project for self-learning, you may not find this project useful unless you're also learning, or are a recruiter evaluating my skills before offering me a job.

The goal of this project is to get much more comfortable with the rust language, so for now it will only use rust's standard library. In the future I will make another project that uses useful crates to be more productive and I will add the link to that repository here.

HTTP is a protocol that can be used in many places, but I'm specifically testing this server by using a web browser to request web pages. The main browser I'm using is Firefox.

# TODO
- [ ] Handle requests for css files
- [ ] Implement keep connection alive
- [ ] Handle concurrent connections

## Done
- [x] Respond with different html files that match different request received
- [x] Parse the headers of requests to recognize the request type and requested URI
- [x] Create HTTP responses with external html files instead of hardcoded strings
- [x] Create the github repository

