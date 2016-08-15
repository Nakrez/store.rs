describe! store_db {
    before_each {
        use env_logger;

        use db;

        let _ = env_logger::init();

        let mut db = db::Database::new();
    }

    it "initial_state" {
        // Root user has no password
        assert!(db.get("/system/users/root/password") == Ok("".into()));

        // Root is in the root group
        assert!(db.exists("/system/groups/root/root").is_ok());

        // Root is in the users group
        assert!(db.exists("/system/groups/users/root").is_ok());
    }

    it "basic_get" {
        // Get a directory is an error
        assert!(db.get("/").is_err());
        // Non existing keys should yield an error
        assert!(db.get("/non_exist").is_err());

        // Verify data after set
        assert!(db.set("/", "test", "value").is_ok());
        assert!(db.get("/test") == Ok("value".into()));
    }

    it "basic_set" {
        assert!(db.get("/non_exist").is_err());
        assert!(db.set("/", "non_exist", "yes").is_ok());
        assert!(db.get("/non_exist").is_ok());
    }

    it "basic_exists" {
        assert!(db.exists("/non_exist").is_err());
        assert!(db.set("/", "non_exist", "yes").is_ok());
        assert!(db.exists("/non_exist").is_ok());
    }

    it "basic_mkdir" {
        assert!(db.exists("/a").is_err());
        assert!(db.mkdir("/", "a").is_ok());
        assert!(db.exists("/a").is_ok());

        assert!(db.exists("/a/b").is_err());
        assert!(db.mkdir("/a", "b").is_ok());
        assert!(db.exists("/a/b").is_ok());

        assert!(db.exists("/a/b/c").is_err());
        assert!(db.mkdir("/a/b", "c").is_ok());
        assert!(db.exists("/a/b/c").is_ok());

        assert!(db.set("/a/b/c", "d", "value").is_ok());
        assert!(db.get("/a/b/c/d") == Ok("value".into()));
    }

    after_each {
    }
}
