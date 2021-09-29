pub struct Pin {
	pub A1:  PinGroup,
	pub A2:  PinGroup,
	pub A3:  PinGroup,
	pub A4:  PinGroup,
	pub A5:  PinGroup,
	pub A6:  PinGroup,
	pub A7:  PinGroup,
	pub A8:  PinGroup,
	pub A9:  PinGroup,
	pub A10:  PinGroup,
	pub A11: PinGroup,
	pub A12: PinGroup,
	pub A13: PinGroup,
	pub A14: PinGroup,
	pub A15: PinGroup,
	pub A16: PinGroup,
	pub A17: PinGroup,
	pub A18: PinGroup,
	pub A19: PinGroup,
	pub A20: PinGroup,
	pub A21: PinGroup,
	pub A22: PinGroup,
	pub A23: PinGroup,
	pub A24: PinGroup,
	pub A25: PinGroup,
	pub A26: PinGroup,
	pub A27: PinGroup,
	pub A28: PinGroup,
	pub A29: PinGroup,
	pub A30: PinGroup,
	pub A31: PinGroup,
	pub A32: PinGroup,
	pub B1:  PinGroup,
	pub B2:  PinGroup,
	pub B3:  PinGroup,
	pub B4:  PinGroup,
	pub B5:  PinGroup,
	pub B6:  PinGroup,
	pub B7:  PinGroup,
	pub B8:  PinGroup,
	pub B9:  PinGroup,
	pub B10:  PinGroup,
	pub B11: PinGroup,
	pub B12: PinGroup,
	pub B13: PinGroup,
	pub B14: PinGroup,
	pub B15: PinGroup,
	pub B16: PinGroup,
	pub B17: PinGroup,
	pub B18: PinGroup,
	pub B19: PinGroup,
	pub B20: PinGroup,
	pub B21: PinGroup,
	pub B22: PinGroup,
	pub B23: PinGroup,
	pub B24: PinGroup,
	pub B25: PinGroup,
	pub B26: PinGroup,
	pub B27: PinGroup,
	pub B28: PinGroup,
	pub B29: PinGroup,
	pub B30: PinGroup,
	pub B31: PinGroup,
	pub B32: PinGroup,
	pub C1:  PinGroup,
	pub C2:  PinGroup,
	pub C3:  PinGroup,
	pub C4:  PinGroup,
	pub C5:  PinGroup,
	pub C6:  PinGroup,
	pub C7:  PinGroup,
	pub C8:  PinGroup,
	pub C9:  PinGroup,
	pub C10:  PinGroup,
	pub C11: PinGroup,
	pub C12: PinGroup,
	pub C13: PinGroup,
	pub C14: PinGroup,
	pub C15: PinGroup,
	pub C16: PinGroup,
	pub C17: PinGroup,
	pub C18: PinGroup,
	pub C19: PinGroup,
	pub C20: PinGroup,
	pub C21: PinGroup,
	pub C22: PinGroup,
	pub C23: PinGroup,
	pub C24: PinGroup,
	pub C25: PinGroup,
	pub C26: PinGroup,
	pub C27: PinGroup,
	pub C28: PinGroup,
	pub C29: PinGroup,
	pub C30: PinGroup,
	pub C31: PinGroup,
	pub C32: PinGroup,
	pub D1:  PinGroup,
	pub D2:  PinGroup,
	pub D3:  PinGroup,
	pub D4:  PinGroup,
	pub D5:  PinGroup,
	pub D6:  PinGroup,
	pub D7:  PinGroup,
	pub D8:  PinGroup,
	pub D9:  PinGroup,
	pub D10:  PinGroup,
	pub D11: PinGroup,
	pub D12: PinGroup,
	pub D13: PinGroup,
	pub D14: PinGroup,
	pub D15: PinGroup,
	pub D16: PinGroup,
	pub D17: PinGroup,
	pub D18: PinGroup,
	pub D19: PinGroup,
	pub D20: PinGroup,
	pub D21: PinGroup,
	pub D22: PinGroup,
	pub D23: PinGroup,
	pub D24: PinGroup,
	pub D25: PinGroup,
	pub D26: PinGroup,
	pub D27: PinGroup,
	pub D28: PinGroup,
	pub D29: PinGroup,
	pub D30: PinGroup,
	pub D31: PinGroup,
	pub D32: PinGroup
}


#[derive(Debug, Copy, Clone)]
pub enum Group {
    Group1 = 0,
    Group2,
    Group3,
    Group4,
}

pub struct PinGroup {
	pub group: Group,
	pub number: u32,
}


pub fn init_pins() -> Pin {
	Pin{
		A1: PinGroup{group: Group::Group1, number: 1},
		A2: PinGroup{group: Group::Group1, number: 2},
		A3: PinGroup{group: Group::Group1, number: 3},
		A4: PinGroup{group: Group::Group1, number: 4},
		A5: PinGroup{group: Group::Group1, number: 5},
		A6: PinGroup{group: Group::Group1, number: 6},
		A7: PinGroup{group: Group::Group1, number: 7},
		A8: PinGroup{group: Group::Group1, number: 8},
		A9: PinGroup{group: Group::Group1, number: 9},
		A10: PinGroup{group: Group::Group1, number: 10},
		A11: PinGroup{group: Group::Group1, number: 11},
		A12: PinGroup{group: Group::Group1, number: 12},
		A13: PinGroup{group: Group::Group1, number: 13},
		A14: PinGroup{group: Group::Group1, number: 14},
		A15: PinGroup{group: Group::Group1, number: 15},
		A16: PinGroup{group: Group::Group1, number: 16},
		A17: PinGroup{group: Group::Group1, number: 17},
		A18: PinGroup{group: Group::Group1, number: 18},
		A19: PinGroup{group: Group::Group1, number: 19},
		A20: PinGroup{group: Group::Group1, number: 20},
		A21: PinGroup{group: Group::Group1, number: 21},
		A22: PinGroup{group: Group::Group1, number: 22},
		A23: PinGroup{group: Group::Group1, number: 23},
		A24: PinGroup{group: Group::Group1, number: 24},
		A25: PinGroup{group: Group::Group1, number: 25},
		A26: PinGroup{group: Group::Group1, number: 26},
		A27: PinGroup{group: Group::Group1, number: 27},
		A28: PinGroup{group: Group::Group1, number: 28},
		A29: PinGroup{group: Group::Group1, number: 29},
		A30: PinGroup{group: Group::Group1, number: 30},
		A31: PinGroup{group: Group::Group1, number: 31},
		A32: PinGroup{group: Group::Group1, number: 32},
		B1: PinGroup{group: Group::Group2, number: 1},
		B2: PinGroup{group: Group::Group2, number: 2},
		B3: PinGroup{group: Group::Group2, number: 3},
		B4: PinGroup{group: Group::Group2, number: 4},
		B5: PinGroup{group: Group::Group2, number: 5},
		B6: PinGroup{group: Group::Group2, number: 6},
		B7: PinGroup{group: Group::Group2, number: 7},
		B8: PinGroup{group: Group::Group2, number: 8},
		B9: PinGroup{group: Group::Group2, number: 9},
		B10: PinGroup{group: Group::Group2, number: 10},
		B11: PinGroup{group: Group::Group2, number: 11},
		B12: PinGroup{group: Group::Group2, number: 12},
		B13: PinGroup{group: Group::Group2, number: 13},
		B14: PinGroup{group: Group::Group2, number: 14},
		B15: PinGroup{group: Group::Group2, number: 15},
		B16: PinGroup{group: Group::Group2, number: 16},
		B17: PinGroup{group: Group::Group2, number: 17},
		B18: PinGroup{group: Group::Group2, number: 18},
		B19: PinGroup{group: Group::Group2, number: 19},
		B20: PinGroup{group: Group::Group2, number: 20},
		B21: PinGroup{group: Group::Group2, number: 21},
		B22: PinGroup{group: Group::Group2, number: 22},
		B23: PinGroup{group: Group::Group2, number: 23},
		B24: PinGroup{group: Group::Group2, number: 24},
		B25: PinGroup{group: Group::Group2, number: 25},
		B26: PinGroup{group: Group::Group2, number: 26},
		B27: PinGroup{group: Group::Group2, number: 27},
		B28: PinGroup{group: Group::Group2, number: 28},
		B29: PinGroup{group: Group::Group2, number: 29},
		B30: PinGroup{group: Group::Group2, number: 30},
		B31: PinGroup{group: Group::Group2, number: 31},
		B32: PinGroup{group: Group::Group2, number: 32},
		C1: PinGroup{group: Group::Group3, number: 1},
		C2: PinGroup{group: Group::Group3, number: 2},
		C3: PinGroup{group: Group::Group3, number: 3},
		C4: PinGroup{group: Group::Group3, number: 4},
		C5: PinGroup{group: Group::Group3, number: 5},
		C6: PinGroup{group: Group::Group3, number: 6},
		C7: PinGroup{group: Group::Group3, number: 7},
		C8: PinGroup{group: Group::Group3, number: 8},
		C9: PinGroup{group: Group::Group3, number: 9},
		C10: PinGroup{group: Group::Group3, number: 10},
		C11: PinGroup{group: Group::Group3, number: 11},
		C12: PinGroup{group: Group::Group3, number: 12},
		C13: PinGroup{group: Group::Group3, number: 13},
		C14: PinGroup{group: Group::Group3, number: 14},
		C15: PinGroup{group: Group::Group3, number: 15},
		C16: PinGroup{group: Group::Group3, number: 16},
		C17: PinGroup{group: Group::Group3, number: 17},
		C18: PinGroup{group: Group::Group3, number: 18},
		C19: PinGroup{group: Group::Group3, number: 19},
		C20: PinGroup{group: Group::Group3, number: 20},
		C21: PinGroup{group: Group::Group3, number: 21},
		C22: PinGroup{group: Group::Group3, number: 22},
		C23: PinGroup{group: Group::Group3, number: 23},
		C24: PinGroup{group: Group::Group3, number: 24},
		C25: PinGroup{group: Group::Group3, number: 25},
		C26: PinGroup{group: Group::Group3, number: 26},
		C27: PinGroup{group: Group::Group3, number: 27},
		C28: PinGroup{group: Group::Group3, number: 28},
		C29: PinGroup{group: Group::Group3, number: 29},
		C30: PinGroup{group: Group::Group3, number: 30},
		C31: PinGroup{group: Group::Group3, number: 31},
		C32: PinGroup{group: Group::Group3, number: 32},
		D1: PinGroup{group: Group::Group4, number: 1},
		D2: PinGroup{group: Group::Group4, number: 2},
		D3: PinGroup{group: Group::Group4, number: 3},
		D4: PinGroup{group: Group::Group4, number: 4},
		D5: PinGroup{group: Group::Group4, number: 5},
		D6: PinGroup{group: Group::Group4, number: 6},
		D7: PinGroup{group: Group::Group4, number: 7},
		D8: PinGroup{group: Group::Group4, number: 8},
		D9: PinGroup{group: Group::Group4, number: 9},
		D10: PinGroup{group: Group::Group3, number: 10},
		D11: PinGroup{group: Group::Group3, number: 11},
		D12: PinGroup{group: Group::Group3, number: 12},
		D13: PinGroup{group: Group::Group3, number: 13},
		D14: PinGroup{group: Group::Group3, number: 14},
		D15: PinGroup{group: Group::Group3, number: 15},
		D16: PinGroup{group: Group::Group3, number: 16},
		D17: PinGroup{group: Group::Group3, number: 17},
		D18: PinGroup{group: Group::Group3, number: 18},
		D19: PinGroup{group: Group::Group3, number: 19},
		D20: PinGroup{group: Group::Group3, number: 20},
		D21: PinGroup{group: Group::Group3, number: 21},
		D22: PinGroup{group: Group::Group3, number: 22},
		D23: PinGroup{group: Group::Group3, number: 23},
		D24: PinGroup{group: Group::Group3, number: 24},
		D25: PinGroup{group: Group::Group3, number: 25},
		D26: PinGroup{group: Group::Group3, number: 26},
		D27: PinGroup{group: Group::Group3, number: 27},
		D28: PinGroup{group: Group::Group3, number: 28},
		D29: PinGroup{group: Group::Group3, number: 29},
		D30: PinGroup{group: Group::Group3, number: 30},
		D31: PinGroup{group: Group::Group3, number: 31},
		D32: PinGroup{group: Group::Group3, number: 32}
	}
}