// Copyright (C) 2020, Edward O'Callaghan.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

pub mod prelude {
    pub use crate::cmd::math::{CRC16, calculate_num_pages};

    pub use crate::cmd::general::GeneralCmd;
    pub use crate::cmd::general::OpModus;

    pub use crate::cmd::logging::LoggingCmd;
    pub use crate::cmd::power::PowerCmd;
    pub use crate::cmd::measure::MeasureCmd;
    pub use crate::cmd::sensor::SensorCmd;
    pub use crate::cmd::algorithm::AlgorithmCmd;
    pub use crate::cmd::airbag::AirbagCmd;

    pub use crate::cmd::swupdate::SWUpdateCmd;
    pub use crate::cmd::swupdate::SWUpdateBootLoaderStates;
    pub use crate::cmd::swupdate::FWData;
}

mod math;

mod general;
mod logging;
mod power;
mod measure;
mod sensor;
mod algorithm;
mod airbag;
mod swupdate;
