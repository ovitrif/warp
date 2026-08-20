# Product Spec: Overwrite an existing launch configuration

**Issue:** [warpdotdev/warp#7778](https://github.com/warpdotdev/warp/issues/7778)
**No in-flight PR.** Implement on the local overlay; do not open a duplicate issue.

## Summary

Saving a launch configuration whose YAML name already exists must not be a
dead-end error. Offer **Overwrite** so the current windows/tabs/panes replace
that file, or let the user go back and pick a different name.

## Problem

`File → Save Launch Configuration` rejects a duplicate name with
“Failed to save. A launch configuration with the same name already exists.”
The only control is the modal close button. Updating a config means
save-as-temp, delete, rename (#7778).

## Goals

- On duplicate name, show a confirmation with **Overwrite** and **Cancel**.
- Overwrite replaces `~/.warp/launch_configurations/<name>.yaml` (OSS:
  `~/.warp-oss/...`) with a fresh snapshot of the current session.
- Cancel returns to the name field; the existing file is untouched.
- Enter on the confirmation overwrites. Escape still closes the modal.

## Non-goals

- In-place YAML editing, rename/delete in the File menu (#8323).
- Cloud / Warp Drive launch configs.
- Auto-overwrite without confirmation.

## User-visible behavior

1. User saves as `default` and `default.yaml` already exists.
2. Modal body becomes a confirmation naming that file. Primary button is
   **Overwrite**; secondary is **Cancel**.
3. Overwrite writes the YAML and shows the existing success state
   (path + Open YAML File).
4. Cancel restores the save form with the typed name still in the editor.
