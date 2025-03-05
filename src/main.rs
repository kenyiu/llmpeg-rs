use clap::Parser;
use genai::chat::{ChatMessage, ChatRequest};
use genai::resolver::{AuthData, AuthResolver};
use genai::{Client, ModelIden};
use std::process::Command;
use log::{debug, error};

#[derive(Parser)]
#[command(
    about = "llmpeg: A natural language wrapper for ffmpeg.",
)]

#[derive(Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Task in natural language
    #[arg(value_name = "NATURAL_LANGUAGE_TASK", help = "Describe the multimedia task (e.g., 'convert screencapture.webm to screencapture.mp4, downscale to 720p')")]
    task: String,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();

    if cli.debug {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();
    }

    debug!("Parsed CLI arguments: {:?}", cli);

    let output = Command::new("ffmpeg")
        .args(["-version"])
        .output();

    let version_info: String = match output {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            debug!("Failed to execute ffmpeg: {}", e);
            "Latest version".to_string()
        }
    };

    debug!("ffmpeg version info:\n{}", version_info);

    // Construct the prompt with examples for the LLM
    let question = format!(
        "You are an assistant that generates ffmpeg commands from natural language descriptions.\n
        The ffmpeg version info is: '{}'
        Here are some examples:\n\n\
        Natural language: convert video.mp4 to audio.mp3\n\
        FFMPEG command: ffmpeg -i video.mp4 -vn -ar 44100 -ac 2 -ab 192k audio.mp3\n\n\
        Natural language: extract frames from video.avi and save them as images in a folder\n\
        FFMPEG command: ffmpeg -i video.avi -f image2 image-%03d.jpg\n\n\
        Natural language: convert the file screencapture.webm to screencapture.mp4, but downscale from 1080p to 720p\n\
        FFMPEG command: ffmpeg -i screencapture.webm -vf scale=1280:720 screencapture.mp4\n\n\
        Now, given the natural language command: '{}', generate the corresponding ffmpeg command.\n\n\
        Only output the command, no explanations or additional text.\n\n\
        Format: command",
        version_info,
        cli.task
    );

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("Answer in one sentence"),
        ChatMessage::user(&question),
    ]);

    // let client = Client::default();

    // check if the key for corresponding model is set
    let env_name = "LLMPEG_API_KEY";
    let model = "gemini-2.0-flash";
    
    let api_key = match std::env::var(env_name) {
        Ok(api_key) => api_key,
        Err(_) => {
            error!("{env_name} environment variable not set");
            std::process::exit(exitcode::CONFIG);
        }
    };

    let auth_resolver = AuthResolver::from_resolver_fn(
		|model_iden: ModelIden| -> Result<Option<AuthData>, genai::resolver::Error> {
			let ModelIden {
				adapter_kind,
				model_name,
			} = model_iden;
			debug!("Custom auth provider for {adapter_kind} (model: {model_name})");

			Ok(Some(AuthData::from_single(api_key)))
		},
	);
    let client = Client::builder().with_auth_resolver(auth_resolver).build();

    let adapter_kind = client.resolve_service_target(model)?.model.adapter_kind;

    debug!("MODEL: {model} ({adapter_kind})");

    debug!("Question:\n{question}");

    let chat_res = client.exec_chat(model, chat_req.clone(), None).await?;
    println!("{}", chat_res.content_text_as_str().unwrap_or("NO ANSWER"));
    Ok(())
}
