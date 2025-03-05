# llmpeg-rs

llmpeg-rs is a natural language wrapper for ffmpeg, allowing users to describe multimedia tasks in natural language and get the corresponding ffmpeg command because we don't want to leave the terminal and just hope the command would work like a charm.

## Features

- Convert natural language descriptions to ffmpeg commands using Gemini API

## Supported OS

Currently, only MacOS is tested on my machine, but it should work on other platforms as well since no OS specific code is used. Let me know if you have any issues.

## Installation

Download the binary from release session. (https://github.com/kenyiu/llmpeg-rs/releases)

## Installation from source

1. Clone the repository:

    ```sh
    git clone https://github.com/kenyiu/llmpeg-rs.git
    cd llmpeg
    ```

2. Install dependencies:

    ```sh
    cargo build
    ```

3. The binary (`llmpeg`) will be built in the `target/debug` directory.

## Usage

1. Set the environment variable for the API key:

    ```sh
    export LLMPEG_API_KEY=your_api_key_here
    ```

2. Run the application with a natural language task description:

    ```sh
    "convert screencapture.webm to screencapture.mp4, downscale to 720p"
    ```

3. Enable debugging information:

    ```sh
    cargo run -- --debug "convert screencapture.webm to screencapture.mp4, downscale to 720p"
    ```

## Command-line Options

```
llmpeg: A natural language wrapper for ffmpeg.

Usage: llmpeg [OPTIONS] <NATURAL_LANGUAGE_TASK>

Arguments:
  <NATURAL_LANGUAGE_TASK>  Describe the multimedia task (e.g., 'convert screencapture.webm to screencapture.mp4, downscale to 720p')

Options:
  -d, --debug    Turn debugging information on
  -h, --help     Print help
  -V, --version  Print version
```

## Roadmap

- [x] Support for Gemini API
- [x] Release Build CI
- [ ] Config file for LLM model name and prompt
- [ ] Support for OPENAI API
- [ ] Support for Ollama API
- [ ] Add test cases

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
