describe! stainless {
    before_each {
        use db;

        let db = db::Database::new();
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
        assert!(db.set("/", "non_exist", "yes").is_ok());
        assert!(db.get("/non_exist").is_ok());
    }

    after_each {
    }
}
