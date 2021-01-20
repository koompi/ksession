use iced::{
   pick_list, button, scrollable, Element, Row, PickList, Scrollable, Button, Text, Align, Length, Container,
};
use crate::gui::{CustomSelect, CustomButton};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct DefaultAppPage { 
   scroll: scrollable::State,
   audio_player: (&'static str, pick_list::State<AudioPlayer>, AudioPlayer, button::State),
   video_player: (&'static str, pick_list::State<VideoPlayer>, VideoPlayer, button::State),
   image_viewer: (&'static str, pick_list::State<ImageViewer>, ImageViewer, button::State),
   web_browser: (&'static str, pick_list::State<WebBrowser>, WebBrowser, button::State),
   term_emulation: (&'static str, pick_list::State<TermEmulation>, TermEmulation, button::State),
   mail_client: (&'static str, pick_list::State<MailClient>, MailClient, button::State),
   calendar: (&'static str, pick_list::State<Calendar>, Calendar, button::State),
}

#[derive(Debug, Clone)]
pub enum DefaultAppMsg {
   // DefAppChanged(usize),
   // DefAppSearchClicked(usize),
   AudioPlayerChanged(AudioPlayer),
   VideoPlayerChanged(VideoPlayer),
   ImageViewerChanged(ImageViewer),
   WebBrowserChanged(WebBrowser),
   TermEmulationChanged(TermEmulation),
   MailClientChanged(MailClient),
   CalendarChanged(Calendar),
   BtnSearchClicked,
}

impl DefaultAppPage {
   pub fn new() -> Self {
      Self {
         scroll: scrollable::State::new(),
         audio_player: ("Audio Player", pick_list::State::default(), AudioPlayer::VLC, button::State::new()),
         video_player: ("Video Player", pick_list::State::default(), VideoPlayer::VLC, button::State::new()),
         image_viewer: ("Image Viewer", pick_list::State::default(), ImageViewer::GPicView, button::State::new()),
         web_browser: ("Web Browser", pick_list::State::default(), WebBrowser::Firefox, button::State::new()),
         term_emulation: ("Terminal Emulation", pick_list::State::default(), TermEmulation::Alacritty, button::State::new()),
         mail_client: ("Mail Client", pick_list::State::default(), MailClient::Thunderbird, button::State::new()),
         calendar: ("Calendar", pick_list::State::default(), Calendar::California, button::State::new()),
      }
   }

   pub fn update(&mut self, msg: DefaultAppMsg) {
      use DefaultAppMsg::*;
      match msg {
         AudioPlayerChanged(val) => self.audio_player.2 = val,
         VideoPlayerChanged(val) => self.video_player.2 = val,
         ImageViewerChanged(val) => self.image_viewer.2 = val,
         WebBrowserChanged(val) => self.web_browser.2 = val,
         TermEmulationChanged(val) => self.term_emulation.2 = val,
         MailClientChanged(val) => self.mail_client.2 = val,
         CalendarChanged(val) => self.calendar.2 = val,
         BtnSearchClicked => println!("Search clicked"),
      }
   }

   pub fn view(&mut self) -> Element<DefaultAppMsg> {
      let DefaultAppPage {
         scroll,
         audio_player,
         video_player,
         image_viewer,
         web_browser,
         term_emulation,
         mail_client,
         calendar,
      } = self;

      let txt_def_apps = Text::new("Default Applications").size(14);
      let audio_player_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", audio_player.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut audio_player.1, &AudioPlayer::ALL[..], Some(audio_player.2), DefaultAppMsg::AudioPlayerChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut audio_player.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let video_player_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", video_player.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut video_player.1, &VideoPlayer::ALL[..], Some(video_player.2), DefaultAppMsg::VideoPlayerChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut video_player.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let image_viewer_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", image_viewer.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut image_viewer.1, &ImageViewer::ALL[..], Some(image_viewer.2), DefaultAppMsg::ImageViewerChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut image_viewer.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let web_browser_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", web_browser.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut web_browser.1, &WebBrowser::ALL[..], Some(web_browser.2), DefaultAppMsg::WebBrowserChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut web_browser.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let term_emulation_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", term_emulation.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut term_emulation.1, &TermEmulation::ALL[..], Some(term_emulation.2), DefaultAppMsg::TermEmulationChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut term_emulation.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let mail_client_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", mail_client.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut mail_client.1, &MailClient::ALL[..], Some(mail_client.2), DefaultAppMsg::MailClientChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut mail_client.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      let calendar_sec = Row::new().spacing(10).align_items(Align::Center)
         .push(Container::new(Text::new(format!("{}:", calendar.0))).width(Length::Units(127)))
         .push(Container::new(PickList::new(&mut calendar.1, &Calendar::ALL[..], Some(calendar.2), DefaultAppMsg::CalendarChanged).width(Length::Fill).style(CustomSelect::Default)).width(Length::Fill))
         .push(Button::new(&mut calendar.3, Text::new("  Search  ")).on_press(DefaultAppMsg::BtnSearchClicked).style(CustomButton::Default));
      
      Scrollable::new(scroll).spacing(12).scroller_width(5).scrollbar_width(7)
         .push(txt_def_apps)
         .push(audio_player_sec)
         .push(video_player_sec)
         .push(image_viewer_sec)
         .push(web_browser_sec)
         .push(term_emulation_sec)
         .push(mail_client_sec)
         .push(calendar_sec)
         .into()
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioPlayer {
   VLC,
   Rhythmbox,
   Clementine,
   Audacious,
   CMUS,
   Sayonara,
   Museeks,
   Spotify,
}

impl AudioPlayer {
   const ALL: [AudioPlayer; 8] = [
      AudioPlayer::VLC,
      AudioPlayer::Rhythmbox,
      AudioPlayer::Clementine,
      AudioPlayer::Audacious,
      AudioPlayer::CMUS,
      AudioPlayer::Sayonara,
      AudioPlayer::Museeks,
      AudioPlayer::Spotify,
   ];
}

impl Display for AudioPlayer {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            AudioPlayer::VLC => "VLC Media Player",
            AudioPlayer::Rhythmbox => "Rhythmbox",
            AudioPlayer::Clementine => "Clementine",
            AudioPlayer::Audacious => "Audacious",
            AudioPlayer::CMUS => "CMUS",
            AudioPlayer::Sayonara => "Sayonara",
            AudioPlayer::Museeks => "Museeks",
            AudioPlayer::Spotify => "Spotify",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoPlayer {
   VLC,
   MPlayer,
   MPV,
   FFPlay,
   GstPlay,
}

impl VideoPlayer {
   const ALL: [VideoPlayer; 5] = [
      VideoPlayer::VLC,
      VideoPlayer::MPlayer,
      VideoPlayer::MPV,
      VideoPlayer::FFPlay,
      VideoPlayer::GstPlay,
   ];
}

impl Display for VideoPlayer {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            VideoPlayer::VLC => "VLC Media Player",
            VideoPlayer::MPlayer => "MPlayer",
            VideoPlayer::MPV => "MPV Player",
            VideoPlayer::FFPlay => "FFPlay Media Player",
            VideoPlayer::GstPlay => "GST Play",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageViewer {
   GPicView,
   Ephoto,
   GalaPic,
   GWenView,
   QuickImgViewer,
}

impl ImageViewer {
   const ALL: [ImageViewer; 5] = [
      ImageViewer::GPicView,
      ImageViewer::Ephoto,
      ImageViewer::GalaPic,
      ImageViewer::GWenView,
      ImageViewer::QuickImgViewer,
   ];
}

impl Display for ImageViewer {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            ImageViewer::GPicView => "GPicView",
            ImageViewer::Ephoto => "Ephoto",
            ImageViewer::GalaPic => "GalaPic",
            ImageViewer::GWenView => "GWenView",
            ImageViewer::QuickImgViewer => "Quick Image Viewer",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebBrowser {
   Firefox,
   Chrome,
   Chromium,
   Brave,
}

impl WebBrowser {
   const ALL: [WebBrowser; 4] = [
      WebBrowser::Firefox,
      WebBrowser::Chrome,
      WebBrowser::Chromium,
      WebBrowser::Brave,
   ];
}

impl Display for WebBrowser {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            WebBrowser::Firefox => "Firefox web browser",
            WebBrowser::Chrome => "Google Chrome",
            WebBrowser::Chromium => "Chromium browser",
            WebBrowser::Brave => "Brave web browser",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermEmulation {
   Alacritty,
   PuTTY,
   Kitty,
   Xterm,
   Terminator,
}

impl TermEmulation {
   const ALL: [TermEmulation; 5] = [
      TermEmulation::Alacritty,
      TermEmulation::PuTTY,
      TermEmulation::Kitty,
      TermEmulation::Xterm,
      TermEmulation::Terminator,
   ];
}

impl Display for TermEmulation {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            TermEmulation::Alacritty => "Alacritty",
            TermEmulation::PuTTY => "PuTTy",
            TermEmulation::Kitty => "Kitty",
            TermEmulation::Xterm => "Xterm",
            TermEmulation::Terminator => "Terminator",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailClient {
   Msmtp,
   Mutt,
   SNail,
   Sup,
   Thunderbird,
}

impl MailClient {
   const ALL: [MailClient; 5] = [
      MailClient::Msmtp,
      MailClient::Mutt,
      MailClient::SNail,
      MailClient::Sup,
      MailClient::Thunderbird,
   ];
}

impl Display for MailClient {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            MailClient::Msmtp => "msmtp",
            MailClient::Mutt => "Mutt",
            MailClient::SNail => "S-nail",
            MailClient::Sup => "Sup",
            MailClient::Thunderbird => "Thunderbird",
         }
      )
   }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Calendar {
   California,
   Osmo,
   Evolution,
   Korganizer,
}

impl Calendar {
   const ALL: [Calendar; 4] = [
      Calendar::California,
      Calendar::Osmo,
      Calendar::Evolution,
      Calendar::Korganizer,
   ];
}

impl Display for Calendar {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            Calendar::California => "California",
            Calendar::Osmo => "Osmo",
            Calendar::Evolution => "Evolution",
            Calendar::Korganizer => "Korganizer",
         }
      )
   }
}