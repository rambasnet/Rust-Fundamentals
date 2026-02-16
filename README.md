# Rust Programming Language Fundamentals

- Jupyter Notebooks, Pdfs, Labs and Exercises for Computer Science Foundational Concepts using Rust Programming Language
- some of the contents are based on the open source textbook: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) and [https://doc.rust-lang.org/reference/](https://doc.rust-lang.org/reference/)

## Who can use this content

### University and high-school coding instructors

Depending on the course level and topics covered, instructors can pick and choose appropriate chapters.

### Self learners

Depending on their skill and interest level, learners can move as swiftly as appropriate through the chapters. Try solving some exercises towards the end of each chapter before moving on to self-assess the mastery of the materials.

## How to use Jupyter Notebook

### Important

In order to learn coding, it's very important to actually type code on your own from scratch and NOT copy paste! You can run provided cells to see the output, follow along and learn from it. However, it's very important that you either start a new notebook or add cells and write your own code from scratch to practice the concepts covered with many similar examples and solve the exercises provided.

### Use Dockerfile and Docker-compose

- the repository comes with Docker settings for rust and jupyter notebook
```bash
bash run-docker.sh
```
- once you're in the docker, run the following command to start Jupyter Notebook

```bash
bash run-jupyter.sh
```

### On a local system or on GitHub Codespaces

To run these notebooks interactively and save your work locally, you need the following environment and programs installed.

- Linux, MacOS or WSL (Ubuntu App) on Windows
- Jupyter Notebook - learning environment
- rust kernerl for Jupyter Notebook
- git client - to use version control
- rust compiler - to compile and run sample codes, solve exercises and labs
- VS Code or any Code Editor to write C++ programs

## Install Rust

- Rust can be installed using the `rustup` tool, which is the recommended way to install Rust.
- It allows you to manage multiple versions of Rust and associated tools easily.
- To install Rust using `rustup`, follow these steps:

1. Open a terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Follow the on-screen instructions to complete the installation process. You can choose the default installation
3. After the installation is complete, you may need to restart your terminal or run the following command to update your PATH environment variable:

```bash
source $HOME/.cargo/env
```

4. To verify that Rust is installed correctly, you can run the following command:

```bash
rustc --version
```

- This should display the version of Rust that you have installed.
- You can also check the version of Cargo, Rust's package manager, by running:

```bash
cargo --version
```

- With Rust installed, you can now start writing and running Rust code!

## Writing Rust in Jupyter Notebooks

- Jupyter Notebooks are a popular tool for interactive programming and data analysis.
- You can use Rust in Jupyter Notebooks by installing the `evcxr_jupyter` kernel, which allows you to write and execute Rust code directly in your notebooks.
- install jupyter notebook if you haven't already:

```bash
pip install jupyter notebook
```

## Install the evcxr_jupyter kernel for Rust in Jupyter Notebooks

- Use Rust's package manager, Cargo, to install the kernel from crates.io.

```bash
 cargo install evcxr_jupyter
```

- After the installation is complete, you need to register the kernel with Jupyter. Run the following command:

```bash
evcxr_jupyter --install
```

- This will set up the Rust kernel for Jupyter Notebooks.
- You can also install VS Code extensions for Jupyter Notebooks to enhance your experience when working with Rust in notebooks

- Or, you can start Jupyter Notebooks by running:

```bash
jupyter notebook
```

- In the Jupyter interface, you can create a new notebook and select "Rust" as the kernel to start writing and executing Rust code in your notebook cells.
- You can use the same Rust syntax and features as you would in a regular Rust program, and the output will be displayed directly in the notebook.

## Content contributors

Ram Basnet, PhD, (rbasnet@coloradomesa.edu) Professor of Computer Science, Colorado Mesa University

## Contributing

Contributions are accepted via pull requests. You can also open issues on bugs, typos or any corrections and suggest improvements on the notebooks and other contents.

## Copyright and License

Permission is granted to copy, distribute and/or modify this document under the terms of the [MIT License](https://opensource.org/licenses/MIT). See LICENSE file for details.
