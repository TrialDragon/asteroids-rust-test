use bevy::{
    color::palettes::{css::BLACK, tailwind::GRAY_50},
    prelude::*,
};
use sickle_ui::prelude::*;

use crate::stats::Score;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), setup_game_over);
    app.add_systems(
        Update,
        (restart_button, title_button).run_if(in_state(GameState::GameOver)),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct RestartButton;

fn restart_button(
    query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct TitleButton;

fn title_button(
    query: Query<&Interaction, (Changed<Interaction>, With<TitleButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Title);
        }
    }
}

fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    // UI root.
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            // Title text.
            column
                .spawn(TextBundle::from("Game Over"))
                .style()
                .margin(UiRect::top(Val::Vh(15.)));

            // Score record.
            column.row(|row| {
                row.spawn(TextBundle::from("Asteroids Hit:"));

                row.spawn(TextBundle::from(score.0.to_string()));
            });

            // Button menu.
            column
                .column(|column| {
                    // Restart button.
                    column
                        .container((ButtonBundle::default(), RestartButton), |button| {
                            button
                                .spawn(TextBundle::from("Restart"))
                                .style()
                                .align_self(AlignSelf::Center)
                                .font_color(BLACK.into());
                        })
                        .style()
                        .background_color(GRAY_50.into())
                        .padding(UiRect::horizontal(Val::Px(20.)));

                    // Title button.
                    column
                        .container((ButtonBundle::default(), TitleButton), |button| {
                            button
                                .spawn(TextBundle::from("Title"))
                                .style()
                                .align_self(AlignSelf::Center)
                                .font_color(BLACK.into());
                        })
                        .style()
                        .background_color(GRAY_50.into())
                        .padding(UiRect::horizontal(Val::Px(20.)));
                })
                .style()
                .height(Val::Percent(10.))
                .justify_content(JustifyContent::SpaceAround)
                .margin(UiRect::bottom(Val::Vh(45.)));
        })
        .style()
        .width(Val::Percent(50.))
        .justify_content(JustifyContent::SpaceBetween)
        .align_items(AlignItems::Center)
        .margin(UiRect::all(Val::Auto))
        .entity_commands()
        .insert(StateScoped(GameState::GameOver));
}
