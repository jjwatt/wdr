    #[test]
    fn test_bookmark_file_path_env_override() {
	let dir = TempDir::new().unwrap();
	let custom_path = dir.path().join("custom_bookmarks.txt");
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &custom_path); }
	let path = bookmark_file_path().unwrap();
	assert_eq!(path, custom_path);
    }

    #[test]
    fn test_load_bookmarks_empty_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join(".bookmarks");
	// Create an empty file.
	let _file = File::create(&file_path).unwrap();
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }
        let result = load_bookmarks(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_add_bookmark_creates_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join(".bookmarks");
	// Create an empty file.
	let mut file = File::create(&file_path).unwrap();
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }
        add_bookmark("test_bookmark").unwrap();
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert!(contents.contains("test_bookmark|"));
    }

    #[test]
    fn test_load_ignores_invalid_lines() {
	let dir = TempDir::new().unwrap();
	let file_path = dir.path().join(".bookmarks");
	let mut file = File::create(&file_path).unwrap();
	writeln!(file, "valid|/path").unwrap();
	writeln!(file, "invalid line").unwrap();
	writeln!(file, "different, delimiter, here").unwrap();
	writeln!(file, "# in case you want comments").unwrap();
	writeln!(file, "").unwrap();

	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }

	let bookmarks = load_bookmarks(&file_path).unwrap();
	println!("{:?}", bookmarks);
	assert_eq!(bookmarks.len(), 1);
	assert_eq!(bookmarks[0].name, "valid");
	assert_eq!(bookmarks[0].path, "/path");
    }

    #[test]
    fn test_find_bookmark_missing() {
	let dir = TempDir::new().unwrap();
	let file_path = dir.path().join(".bookmarks");
	File::create(&file_path).unwrap();
	unsafe { std::env::set_var("WDC_BOOKMARK_FILE", &file_path); }
	let result = find_bookmark("nonexistent").unwrap();
	assert!(result.is_none());
    }
