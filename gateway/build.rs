fn main() -> Result<(), Box<dyn std::error::Error>> {
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
