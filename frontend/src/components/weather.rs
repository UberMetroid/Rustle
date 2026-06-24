// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

#![allow(deprecated, dead_code, unused_imports, unused_variables)]

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct WeatherContainerProps {
    pub theme: String,
    pub is_active: bool,
}

#[function_component(WeatherContainer)]
pub fn weather_container(_props: &WeatherContainerProps) -> Html {
    html! {}
}
