# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- **Breaking change:** removed the constants `COMPACT_NOTE_SIZE`,
  `NOTE_PLAINTEXT_SIZE`, and `ENC_CIPHERTEXT_SIZE` as they are now
  implementation spesific (located in `orchard` and `sapling-crypto` crates).
- Generalized the note plaintext size to support variable sizes by adding the
  abstract types `NotePlaintextBytes`, `NoteCiphertextBytes`,
  `CompactNotePlaintextBytes`, and `CompactNoteCiphertextBytes` to the `Domain`
  trait.
- Removed the separate `NotePlaintextBytes` type definition (as it is now an
  associated type).
- Added new `parse_note_plaintext_bytes`, `parse_note_ciphertext_bytes`, and
  `parse_compact_note_plaintext_bytes` methods to the `Domain` trait.
- Updated the `note_plaintext_bytes` method of the `Domain` trait to return the
  `NotePlaintextBytes` associated type.
- Updated the `encrypt_note_plaintext` method of `NoteEncryption` to return the
  `NoteCiphertextBytes` associated type of the `Domain` instead of the explicit
  array.
- Updated the `enc_ciphertext` method of the `ShieldedOutput` trait to return an
  `Option` of a reference instead of a copy.
- Added a new `note_bytes` module with helper trait and struct to deal with note
  bytes data with abstracted underlying array size.
  
## [0.4.0] - 2023-06-06
### Changed
- The `esk` and `ephemeral_key` arguments have been removed from 
  `Domain::parse_note_plaintext_without_memo_ovk`. It is therefore no longer
  necessary (or possible) to ensure that `ephemeral_key` is derived from `esk`
  and the diversifier within the note plaintext. We have analyzed the safety of
  this change in the context of callers within `zcash_note_encryption` and
  `orchard`. See https://github.com/zcash/librustzcash/pull/848 and the
  associated issue https://github.com/zcash/librustzcash/issues/802 for
  additional detail.

## [0.3.0] - 2023-03-22
### Changed
- The `recipient` parameter has been removed from `Domain::note_plaintext_bytes`.
- The `recipient` parameter has been removed from `NoteEncryption::new`. Since 
  the `Domain::Note` type is now expected to contain information about the
  recipient of the note, there is no longer any need to pass this information
  in via the encryption context.

## [0.2.0] - 2022-10-13
### Added
- `zcash_note_encryption::Domain`:
  - `Domain::PreparedEphemeralPublicKey` associated type.
  - `Domain::prepare_epk` method, which produces the above type.

### Changed
- MSRV is now 1.56.1.
- `zcash_note_encryption::Domain` now requires `epk` to be converted to
  `Domain::PreparedEphemeralPublicKey` before being passed to
  `Domain::ka_agree_dec`.
- Changes to batch decryption APIs:
  - The return types of `batch::try_note_decryption` and
    `batch::try_compact_note_decryption` have changed. Now, instead of
    returning entries corresponding to the cartesian product of the IVKs used for
    decryption with the outputs being decrypted, this now returns a vector of
    decryption results of the same length and in the same order as the `outputs`
    argument to the function. Each successful result includes the index of the
    entry in `ivks` used to decrypt the value.

## [0.1.0] - 2021-12-17
Initial release.
