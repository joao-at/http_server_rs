# Introduction

An HTTP server written in rust. (Yes, another one, it's an amazing starting project to get confortable with any programming language)

This is a project for self-learning, you may not find this project usefull unless you're also learning, or are a recruiter evaluating my skills before offering me a job.

The goal of this project is to get much more confortable with the rust language, so for now it will only use rust's standard library. In the future I will make another project that uses usefull crates to be more productive and I will add the link to that repository here.

HTTP is a protocol that can be used in many places, but I'm specifically testing this server by using a web browser to request web pages. The main browser I'm using is Firefox.

# TODO
- [x] Create the github repository
- [ ] Create HTTP responses with external html files instead of hardcoded strings
- [ ] Parse the headers of requests to recognise the request type and requested URI
- [ ] Respond with different html files that match different request received
- [ ] Implement keep connection alive
- [ ] Handle concurrent connections
