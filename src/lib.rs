#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use ark_pallas::{constraints::GVar, Fq, Projective};
    use ark_r1cs_std::{alloc::AllocVar, R1CSVar, ToBytesGadget};
    use ark_relations::r1cs::ConstraintSystem;
    use ark_serialize::CanonicalSerialize;
    use ark_std::UniformRand;

    #[test]
    fn test_point_to_bytes() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<Fq>::new_ref();

        let point = Projective::rand(&mut rng);
        let pointVar = GVar::new_witness(cs.clone(), || Ok(point)).unwrap();

        let mut point_bytes = Vec::new();
        point.serialize_uncompressed(&mut point_bytes).unwrap();

        let pointVar_bytes = pointVar.to_bytes().unwrap();

        assert_eq!(pointVar_bytes.value().unwrap(), point_bytes);
        // They differ in the last byte, the one used to indicate if is point at infinity.
    }
}
