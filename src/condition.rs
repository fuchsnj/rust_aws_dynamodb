pub struct Condition{
	data: String
}
impl Condition{
	fn new(data: &str) -> Condition{
		Condition{
			data: data.to_owned()
		}
	}
	pub fn attribute_exists(attr: &str) -> Condition{
		Condition::new(&format!("attribute_exists({})", attr))
	}
	pub fn attribute_not_exists(attr: &str) -> Condition{
		Condition::new(&format!("attribute_not_exists({})", attr))
	}
	pub fn to_raw_string(&self) -> String{
		self.data.clone()
	}
}

/*
trait ToAttribute{
	fn to_attribute(&self) -> Attribute;
}
impl ToAttribute for Attribute{
	fn to_attribute(&self) -> Attribute{
		self.clone()
	}
}
impl ToAttribute for &str{
	fn to_attribute(&self) -> Attribute{
		Attribute(self.to_owned())
	}
}


#[derive(Clone)]
struct Attribute(String);
impl Attribute{
	fn get_name(&self) -> String{
		self.0.clone()
	}
}*/