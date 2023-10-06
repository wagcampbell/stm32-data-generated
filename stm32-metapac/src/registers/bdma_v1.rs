
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ch",
            extends: None,
            description: Some("Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some("DMA channel configuration register (DMA_CCR)"),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cr"),
                    }),
                },
                BlockItem {
                    name: "ndtr",
                    description: Some("DMA channel 1 number of data register"),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Ndtr"),
                    }),
                },
                BlockItem {
                    name: "par",
                    description: Some("DMA channel 1 peripheral address register"),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: None,
                    }),
                },
                BlockItem {
                    name: "mar",
                    description: Some("DMA channel 1 memory address register"),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: None,
                    }),
                },
            ],
        },
        Block {
            name: "Dma",
            extends: None,
            description: Some("DMA controller"),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some("DMA interrupt status register (DMA_ISR)"),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Isr"),
                    }),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some("DMA interrupt flag clear register (DMA_IFCR)"),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Write,
                        bit_size: 32,
                        fieldset: Some("Isr"),
                    }),
                },
                BlockItem {
                    name: "ch",
                    description: Some("Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 20 })),
                    byte_offset: 8,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("DMA interrupt status register (DMA_ISR)"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gif",
                    description: Some("Channel 1 Global interrupt flag"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: None,
                },
                Field {
                    name: "tcif",
                    description: Some("Channel 1 Transfer Complete flag"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: None,
                },
                Field {
                    name: "htif",
                    description: Some("Channel 1 Half Transfer Complete flag"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: None,
                },
                Field {
                    name: "teif",
                    description: Some("Channel 1 Transfer Error flag"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ndtr",
            extends: None,
            description: Some("DMA channel 1 number of data register"),
            bit_size: 32,
            fields: &[Field {
                name: "ndt",
                description: Some("Number of data to transfer"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("DMA channel configuration register (DMA_CCR)"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("Channel enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcie",
                    description: Some("Transfer complete interrupt enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "htie",
                    description: Some("Half Transfer interrupt enable"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teie",
                    description: Some("Transfer error interrupt enable"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir",
                    description: Some("Data transfer direction"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dir"),
                },
                Field {
                    name: "circ",
                    description: Some("Circular mode"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Circ"),
                },
                Field {
                    name: "pinc",
                    description: Some("Peripheral increment mode"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Inc"),
                },
                Field {
                    name: "minc",
                    description: Some("Memory increment mode"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Inc"),
                },
                Field {
                    name: "psize",
                    description: Some("Peripheral size"),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Size"),
                },
                Field {
                    name: "msize",
                    description: Some("Memory size"),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Size"),
                },
                Field {
                    name: "pl",
                    description: Some("Channel Priority level"),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Pl"),
                },
                Field {
                    name: "mem2mem",
                    description: Some("Memory to memory mode"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Memmem"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Inc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Increment mode disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Increment mode enabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Memmem",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Memory to memory mode disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Memory to memory mode enabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Size",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some("8-bit size"),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some("16-bit size"),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some("32-bit size"),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some("Low priority"),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some("Medium priority"),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("High priority"),
                    value: 2,
                },
                EnumVariant {
                    name: "VERYHIGH",
                    description: Some("Very high priority"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Circ",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Circular buffer disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Circular buffer enabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FROMPERIPHERAL",
                    description: Some("Read from peripheral"),
                    value: 0,
                },
                EnumVariant {
                    name: "FROMMEMORY",
                    description: Some("Read from memory"),
                    value: 1,
                },
            ],
        },
    ],
};
