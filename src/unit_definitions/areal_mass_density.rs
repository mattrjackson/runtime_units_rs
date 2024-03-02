//! Areal mass density (base unit kilogram per square meter, m⁻² · kg).
use crate::{prefix, quantity};
quantity! {
    /// Areal mass density (base unit kilogram per square meter, m⁻² · kg).
    quantity: ArealMassDensity; "areal mass density";
    /// Dimension of areal mass density, L⁻²M (base unit kilogram per square meter, m⁻² · kg).
    dimension: ISQ<
        N2,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_per_square_meter: prefix!(none); "kg/m²", "kilogram per square meter",
            "kilograms per square meter";
        @gram_per_square_meter: prefix!(milli); "g/m²", "gram per square meter",
            "grams per square meter";
        @gram_per_square_centimeter: prefix!(milli) / prefix!(centi) / prefix!(centi); "g/cm²",
            "gram per square centimeter", "grams per square centimeter";

        @ounce_per_square_foot: 2.834_952_E-2 / 9.290_304_E-2; "oz/ft²", "ounce per square foot",
            "ounces per square foot";
    }
}

#[cfg(test)]
mod tests {
    use crate::{units::{MassUnit, ArealMassDensityUnit, LengthUnit}, units_base::Unit};
    
    #[test]
    fn check_dimension() {
        assert_eq!(ArealMassDensityUnit::unit(),  MassUnit::unit() / LengthUnit::unit().powi(2));
    }

   #[test]
    fn check_units() {
        test_unit(ArealMassDensityUnit::kilogram_per_square_meter, LengthUnit::meter, MassUnit::kilogram);
        test_unit(ArealMassDensityUnit::gram_per_square_meter, LengthUnit::meter, MassUnit::gram);
        test_unit(ArealMassDensityUnit::gram_per_square_centimeter, LengthUnit::centimeter, MassUnit::gram);
        test_unit(ArealMassDensityUnit::ounce_per_square_foot, LengthUnit::foot, MassUnit::ounce);

    }
    fn test_unit(value: ArealMassDensityUnit, length: LengthUnit, mass: MassUnit) {
        assert_eq!(Into::<Unit>::into(value), Into::<Unit>::into(mass) / Into::<Unit>::into(length).powi(2));
    }
}