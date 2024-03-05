//! Electric Flux (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Electric Flux (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    quantity: ElectricFlux; "electric flux";
    /// Dimension of electric flux, L³MT⁻³I⁻¹ (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    dimension: ISQ<
        P3,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @volt_meter: prefix!(none); "V ⋅ m", "volt meter", "volt meters";
        @volt_centimeter: prefix!(none) * prefix!(centi); "V ⋅ cm", "volt centimeter",
            "volt centimeters";
    }
}

#[cfg(test)]
mod test {
    use crate::{unit_definitions::{electric_flux::ElectricFluxUnit, electric_potential::ElectricPotentialUnit}, units::LengthUnit, units_base::Unit};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricFluxUnit::unit_base(), ElectricPotentialUnit::unit_base() * LengthUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricFluxUnit::volt_meter, LengthUnit::meter, ElectricPotentialUnit::volt);
        test_unit(ElectricFluxUnit::volt_centimeter, LengthUnit::centimeter, ElectricPotentialUnit::volt);
    }
    fn test_unit(ef: ElectricFluxUnit, length: LengthUnit, potential: ElectricPotentialUnit)
    {
        assert_eq!(Into::<Unit>::into(ef), Into::<Unit>::into(potential) * Into::<Unit>::into(length));
    }
}
