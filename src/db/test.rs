describe! stainless {
    before_each {
        use db;

        let mut __root_pwd = Vec::new();

        __root_pwd.extend_from_slice("baptiste".as_bytes());

        let db = db::Database::new(__root_pwd);
    }

    it "basic_get" {
        db.get("/users");
    }

    after_each {
    }
}
