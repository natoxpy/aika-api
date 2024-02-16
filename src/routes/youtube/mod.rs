use actix_web::{web, Scope, get, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct YoutubeContent {
    pub title: String,
    pub duration: usize,
    pub channel: String,
    pub audio_url: Option<String>,
    pub audio_size: Option<f64>,
}

pub fn get_youtube_audios(url: &str) -> YoutubeContent {
    let yt = youtube_dl::YoutubeDl::new(url).socket_timeout("1").run().unwrap();
    let vid = yt.clone().into_single_video().unwrap();

    let recommended_formats = vid.format.unwrap();
    let formats_split = recommended_formats.split("+").collect::<Vec<&str>>();
    let audio_format = formats_split.get(1).unwrap().to_string();

    let mut yt_content = YoutubeContent {
        title: vid.title.unwrap(),
        duration: vid.duration.unwrap().as_u64().unwrap() as usize,
        channel: vid.channel.unwrap(),
        ..Default::default()
    };

    for format in vid.formats.unwrap().iter() {
        let format_str = &format.format.clone().unwrap();
        if format_str == &audio_format {
            yt_content.audio_url = Some(format.url.clone().unwrap());
            yt_content.audio_size = Some(format.filesize.clone().unwrap());
        }
    }
    
    yt_content
}

#[get("/download/{youtube_url}")]
async fn get_youtube(spath: web::Path<String>) -> impl Responder {
    let yt_url = spath.into_inner();

    println!("{}", yt_url);

    let yt = get_youtube_audios(&yt_url);

    serde_json::to_string(&yt)
}

pub fn scope() -> Scope {
    web::scope("/youtube").service(get_youtube)
}
