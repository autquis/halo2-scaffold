use clap::Parser;
use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use halo2_base::gates::{GateChip, GateInstructions};
use halo2_base::utils::ScalarField;
use halo2_base::AssignedValue;
#[allow(unused_imports)]
use halo2_base::{
    Context,
    QuantumCell::{Constant, Existing, Witness},
};
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub base: String, // field element, but easier to deserialize as a string
    pub exp: String, // field element, but easier to deserialize as a string
    pub res: String, // field element, but easier to deserialize as a string
}

// this algorithm takes a public input base, exp, and res, and checks if base^exp==res.
fn exponentiation_check<F: ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    _make_public: &mut Vec<AssignedValue<F>>,
) {
    let (base, exp, res) = (
        F::from_str_vartime(&input.base).expect("deserialize field element should not fail"),
        F::from_str_vartime(&input.exp).expect("deserialize field element should not fail"),
        F::from_str_vartime(&input.res).expect("deserialize field element should not fail"),
    );
    
    let ctx = builder.main(0);

    let base = ctx.load_witness(base);
    let exp = ctx.load_witness(exp);

    let gate = GateChip::<F>::default();

    let compute = gate.pow_var(ctx, base, exp, F::NUM_BITS as usize);

    let val_assigned =
        ctx.assign_region_last([Existing(compute), Constant(F::ZERO), Constant(F::ONE), Witness(res)], [0]);

    println!("computed result: {:?}", res);
    println!("assigned result: {:?}", val_assigned.value());
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    run(exponentiation_check, args);
}
