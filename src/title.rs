use bevy::{color::palettes::{css::BLACK, tailwind::GRAY_50}, prelude::*};
use sickle_ui::prelude::*;

use crate::GameState;

// TODO: This all needs proper styling,
// but no time for that at this point.
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Title), setup_title);
    app.add_systems(Update, (exit_button, play_button).run_if(in_state(GameState::Title)));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayButton;

fn play_button(
    query: Query<
        &Interaction,
        (Changed<Interaction>, With<PlayButton>)
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        };
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ExitButton;

fn exit_button(
    query: Query<
        &Interaction,
        (Changed<Interaction>, With<ExitButton>)
    >,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            exit_event_writer.send(AppExit::Success);
        }
    }
}

fn setup_title(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|column| {
            column.spawn(TextBundle::from("Asteroids")).style()
                .margin(UiRect::top(Val::Vh(15.)));
            column.column(|column| {
                column.container((
                    ButtonBundle::default(),
                    PlayButton,
                ), |button| {
                    button.spawn(TextBundle::from("Play")).style()
                        .font_color(BLACK.into());
                }).style()
                    .background_color(GRAY_50.into())
                    .padding(UiRect::horizontal(Val::Px(20.)));
                column.container((
                    ButtonBundle::default(),
                    ExitButton,
                ), |button| {
                    button.spawn(TextBundle::from("Exit")).style()
                        .font_color(BLACK.into());
                }).style()
                    .background_color(GRAY_50.into())
                    .padding(UiRect::horizontal(Val::Px(20.)));
            }).style()
                .height(Val::Percent(10.))
                .justify_content(JustifyContent::SpaceAround)
                .margin(UiRect::bottom(Val::Vh(45.)));
    }).style()
        .width(Val::Px(50.0))
        .justify_content(JustifyContent::SpaceBetween)
        .align_items(AlignItems::Center)
        .margin(UiRect::all(Val::Auto))
        .entity_commands()
        .insert(StateScoped(GameState::Title));
}
