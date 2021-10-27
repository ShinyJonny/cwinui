pub struct Screen {
    buffer: Vec<Vec<u8>>,
}

impl Screen {
    pub fn new() -> Self // TODO
    {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn init(&mut self, cols: usize, rows: usize)
    {
        self.buffer.reserve(rows);

        for _row in 0..cols
        {
            self.buffer.push(vec![0; cols])
        }

        let (sup_y, sup_x) = self.supported_max_size();
        if (sup_y < rows || sup_x < cols)
        {
            self.display_error(format!("window too small. min size: {}x{}", cols, rows).as_str());
            std::thread::sleep(std::time::Duration::from_millis(5000));
        }
    }

    pub fn refresh(&self) // TODO
    {
    }

    pub fn size(&self) -> (usize, usize)
    {
        if self.buffer.len() > 0
        {
            (
                self.buffer.len(),
                self.buffer[0].len()
            )
        }
        else
        {
            (0, 0)
        }
    }

    fn supported_max_size(&self) -> (usize, usize) // TODO
    {
        (0, 0)
    }

    fn display_error(&mut self, msg: &str) // TODO
    {
    }
}
