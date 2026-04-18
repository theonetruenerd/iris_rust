// let dma_channel = peripherals.DMA_CH0;
// let (_, tx_descriptors) =
// use esp_hal::{dma_buffers, dma_descriptors};
//
// dma_descriptors!(32000, 32000);
// let (_,_, tx_buffers,_) = dma_buffers!(32000);
// let i2s = I2s::new(
// peripherals.I2S0,
// dma_channel,
// Config::new_tdm_philips()
// .with_sample_rate(Rate::from_hz(44100))
// .with_data_format(DataFormat::Data16Channel16)
// .with_channels(Channels::STEREO)
// );
//
// let mut i2s_tx = i2s
// .unwrap().i2s_tx
// .with_bclk(peripherals.GPIO41)
// .with_ws(peripherals.GPIO43)
// .build(tx_descriptors);
//
// let mut transfer = i2s_tx.write_dma_circular(tx_buffers);
