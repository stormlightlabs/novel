# Novel.rs

Novel writing software built with ~~GTK4 and Rust (Relm4)~~ Iced & Rust. Aiming to be
an IDE for your writing.

## Development

### Crates

- `novelrs-core` (`/core`): Core domain model and logic
- `app` (`/app`): ~~GTK4~~ Iced application containing UI components
- `export` (`/export`): Exporting functionality to EPUB, PDF, Docx
- `markdown` (`/markdown`): Markdown processing (parsing and rendering)
- `store` (`/store`): Git & Dropbox/Cloud integration

### TODO

- Minimal UI skeleton -> [Issue #1](https://github.com/stormlightlabs/novel/issues/1)
- Basic text editor -> [Issue #2](https://github.com/stormlightlabs/novel/issues/2)
- File I/O -> [Issue #6](https://github.com/stormlightlabs/novel/issues/6)
- File Browser -> [Issue #15](https://github.com/stormlightlabs/novel/issues/15)
- Initial integration tests -> [Issue #16](https://github.com/stormlightlabs/novel/issues/16)
- Enhance text editor -> [Issue #17](https://github.com/stormlightlabs/novel/issues/17)
- Document management -> [Issue #18](https://github.com/stormlightlabs/novel/issues/18)
- Document statistics -> [Issue #19](https://github.com/stormlightlabs/novel/issues/19)
- [ ] File browser
    - [ ] Implement directory navigation
    - [ ] Add document listing
    - [ ] Create new folder functionality
- [ ] Configuration system
    - [ ] Create config file structure
    - [ ] Implement settings loading/saving
    - [ ] Add basic preferences UI
- [ ] Basic Git detection
    - [ ] Detect if project is in a Git repository
    - [ ] Display repository status (clean/dirty)
    - [ ] Show current branch name
- [ ] Git operations
    - [ ] Add files to Git staging
    - [ ] Create commit functionality
    - [ ] Implement push/pull operations
- [ ] Git history view
    - [ ] Show commit history for project
    - [ ] Display commit details
    - [ ] Implement file history view
- [ ] Git branching
    - [ ] Create new branch functionality
    - [ ] Switch between branches
    - [ ] Merge branches
- [ ] Auto-save and commit
    - [ ] Add timed auto-save feature
    - [ ] Create auto-commit option
    - [ ] Add commit message templates
- [ ] Dropbox authentication
    - [ ] Implement OAuth flow for Dropbox
    - [ ] Store and manage authentication tokens
    - [ ] Add login/logout functionality
- [ ] Dropbox operations
    - [ ] Upload files to Dropbox
    - [ ] Download files from Dropbox
    - [ ] List Dropbox directories
- [ ] Dropbox synchronization
    - [ ] Two-way synchronization
    - [ ] Add conflict resolution
    - [ ] Create automatic background sync
- [ ] Dropbox integration UI
    - [ ] Add Dropbox status indicator
    - [ ] Create sync progress display
    - [ ] Implement Dropbox file browser
- [ ] Offline functionality
    - [ ] Implement offline editing
    - [ ] Create change tracking for offline changes
    - [ ] Add synchronization queue
- [ ] Project Structure & Organization
    - [ ] Implement chapter organization
    - [ ] Add character tracking
    - [ ] Create notes organization
- [ ] Project templates
    - [ ] Implement template system
    - [ ] Add novel template
    - [ ] Add short story template
- [ ] Project navigation
    - [ ] Create outline view
    - [ ] Implement drag-and-drop reordering
    - [ ] Add search functionality
- [ ] Metadata management
    - [ ] Add tagging system
    - [ ] Create custom metadata fields
    - [ ] Implement filtering by metadata
- [ ] Create multi-file operations
    - [ ] Implement batch file operations
    - [ ] Add multi-file search and replace
    - [ ] Create file consolidation
- [ ] Implement Markdown analysis
    - [ ] Add readability metrics
    - [ ] Create repetition detection
    - [ ] Implement style suggestions
- [ ] Export functionality
    - [ ] Create PDF export
    - [ ] Implement EPUB generation
    - [ ] Add DOCX export
- [ ] More statistics
    - [ ] Add writing session tracking
    - [ ] Implement goals and progress
    - [ ] Create statistics visualization
- [ ] Add collaboration features
    - [ ] Implement Git/Dropbox-based collaboration
    - [ ] Add comments and annotations
    - [ ] Create change tracking
- [ ] Create backup system
    - [ ] Implement automatic backups
    - [ ] Add cloud backup integration
    - [ ] Create restore functionality
