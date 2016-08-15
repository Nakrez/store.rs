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
    }

    after_each {
    }
}
