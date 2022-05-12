use rppal::i2c::I2c;

const ADDR_I2C: u16 = 0x68;
const REG_CTRL_HUM: u8 = 0xF2;
const REG_ADC_VALUE: u8 = 0xF7;

fn acquire() -> Result<u8, rppal::i2c::Error> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(ADDR_I2C)?;
    i2c.smbus_write_byte(REG_CTRL_HUM, 1u8)?;
    let data: u8 = i2c.smbus_read_byte(REG_ADC_VALUE)?;
    Ok(data)
}

fn main() {
    let result = acquire();
    match result {
        Ok(v) => println!("First byte: {}", v),
        Err(e) => println!("Error: {}", e),
    };
}