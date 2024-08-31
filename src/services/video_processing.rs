use reqwest::Client;
use std::process::Command;

pub fn add_audio_to_video(input_video: &str, audio_file: &str, output_video: &str) -> String {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            input_video,
            "-i",
            audio_file,
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            output_video,
        ])
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        output_video.to_string()
    } else {
        eprintln!("FFmpeg error: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to add audio to video");
    }
}

pub async fn post_to_instagram(video_url: &str, thumbnail_url: &str, caption: &str) {
    println!(
        "Posted video: {}, thumbnail: {} with caption: {}",
        video_url, thumbnail_url, caption
    );
    let client = Client::new();
    client
        .post("https://api.instagram.com/v1/media")
        .header("Authorization", "Bearer YOUR_ACCESS_TOKEN")
        .json(&serde_json::json!({
            "video_url": video_url,
            "thumbnail_url": thumbnail_url,
            "caption": caption,
        }))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    println!("Posted video to Instagram");
}
