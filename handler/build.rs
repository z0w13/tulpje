#[cfg(all(feature = "amqp-amqprs", feature = "amqp-lapin"))]
compile_error!(
    "can only pick one amqp implementation, `amqp-amqprs` and `amqp-lapin` are mutually exclusive"
);

// generated by `sqlx migrate build-script`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    // vergen
    vergen_gitcl::Emitter::default()
        .add_instructions(&vergen_gitcl::BuildBuilder::all_build()?)?
        .add_instructions(
            &vergen_gitcl::GitclBuilder::default()
                .all()
                .sha(true)
                .dirty(true)
                .commit_author_name(false)
                .commit_author_email(false)
                .build()?,
        )?
        .emit()?;

    Ok(())
}
