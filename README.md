<a name="readme-top"></a>
# README.md

[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/AchroDev/echto">
    <img src ="images/echto-logo.svg" alt="Logo" width="200" height="200">
  </a>
<h3 align="center"> Echto - Text Editor ~ AchroDev </h3>

  <p align="center">
    This is a custom text editor I am currently writing while following the guide in the acknowledgements. Please feel free to clone the repository and test it out.
    <br />
  </p>
</div>

> [!IMPORTANT]  
> Echo has been fixed and now takes input again. Due to a previous logical error in row.rs[row-rs], my `insert` func failed to actually add the character to the row's string, instead only increasing the row's internal length counter. That logical error is what caused Echto to not take input. I've still noticed some hanging when opening certain files, ones I believe are directly related to Echto's source code. Otherwise, I've tested on other fairly large files without issue.

<!-- ABOUT THE PROJECT -->
## About The Project
![MadeForBadge][made-for-link]
<a href='https://ko-fi.com/R6R3WKVOY' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi3.png?v=3' border='0' alt='Buy Me a Coffee at ko-fi.com' />
</a>

![EchtoSS1][screenshot]
![EchtoSS2][screenshot2]

# 

## Table of Contents

> 1. [Source][source]   
> 2. [Instructions][instructions] 
> 3. [Acknowledgements][acknowledgements]  
#

## Instructions
>
> Find:  
> `Ctrl-F`  
> Save:  
> `Ctrl+S`  
> Quit:  
> `Ctrl+Q`


<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Hecto: Build your own text editor][hecto-guide]
* [README template][readme-template]
* [Rust-lang][rust-lang]
* [The Rust Programming Language (Book)][rust-book]
* [Same as above but by Brown University with quizzes built-in][rust-book-brownuni]

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[license-shield]: https://img.shields.io/github/license/AchroDev/AchroDev.svg?style=for-the-badge
[license-url]: https://github.com/AchroDev/echto/blob/main/LICENSE.txt
[made-for-link]: https://img.shields.io/badge/GNU%20Bash-4EAA25?style=for-the-badge&logo=GNU%20Bash&logoColor=white
[source]: /src
[screenshot]: /images/screenshot.png
[screenshot2]: /images/screenshot2.png
[row-rs]: /src/row.rs
[hecto-guide]: https://archive.flenker.blog/hecto/
[readme-template]: https://github.com/othneildrew/Best-README-Template
[rust-lang]: https://www.rust-lang.org/
[rust-book]: https://doc.rust-lang.org/stable/book/
[rust-book-brownuni]: https://rust-book.cs.brown.edu/
[instructions]: https://github.com/AchroDev/echto?tab=readme-ov-file#instructions
[Acknowledgements]: https://github.com/AchroDev/echto?tab=readme-ov-file#acknowledgements
