# Tech Spec: Overwrite an existing launch configuration

**Product spec:** `specs/GH7778/product.md`
**Issue:** [warpdotdev/warp#7778](https://github.com/warpdotdev/warp/issues/7778)

## Context

`WarpConfig::save_new_launch_config` refuses to write if
`launch_configs_dir()/<name>.yaml` exists. The save modal maps that error to
`SaveState::Failure` and shows a message with no action except close.

`fs::File::create` already truncates, so overwrite is the exists-check plus
the same write path.

## Changes

- `save_new_launch_config(..., overwrite: bool)` — skip the exists error when
  `overwrite` is true.
- `SaveState::ConfirmOverwrite` — duplicate name is a prompt, not a failure.
- `LaunchConfigSaveAction::Overwrite` / `CancelOverwrite`.
- Enter on the prompt overwrites; Cancel restores `NotSaved`.

## Files

- `app/src/user_config/native.rs`
- `app/src/launch_configs/save_modal.rs`
