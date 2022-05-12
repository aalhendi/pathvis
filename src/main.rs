use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Clone, Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideoDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
            <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
                        }
        })
        .collect()
}

//use website_test::tutorial::Video; // replace with your own path

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "One".to_string(),
            speaker: "SpeakerOne".to_string(),
            url: "Url.com".to_string(),
        },
        Video {
            id: 2,
            title: "Two".to_string(),
            speaker: "SpeakerTwo".to_string(),
            url: "url.com".to_string(),
        },
        Video {
            id: 3,
            title: "Three".to_string(),
            speaker: "SpeakerThree".to_string(),
            url: "Url.com".to_string(),
        },
        Video {
            id: 4,
            title: "Four".to_string(),
            speaker: "SpeakerFour".to_string(),
            url: "Url.web".to_string(),
        },
    ];

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
        <h1>{ "Hello World" }</h1>
            <VideosList videos={videos} on_click={on_video_select.clone()} />
            {for details}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
