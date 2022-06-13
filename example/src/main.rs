use std::io;

// use arrayvec::{ArrayString, ArrayVec};
use uhid_virt::{Bus, CreateParams, UHIDDevice};

fn main() {
    // let rd_data = vec![
    //     0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    //     0x09, 0x02, /* USAGE (Mouse) */
    //     0xa1, 0x01, /* COLLECTION (Application) */
    //     0x09, 0x01, /* USAGE (Pointer) */
    //     0xa1, 0x00, /* COLLECTION (Physical) */
    //     0x85, 0x01, /* REPORT_ID (1) */
    //     0x05, 0x09, /* USAGE_PAGE (Button) */
    //     0x19, 0x01, /* USAGE_MINIMUM (Button 1) */
    //     0x29, 0x03, /* USAGE_MAXIMUM (Button 3) */
    //     0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    //     0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    //     0x95, 0x03, /* REPORT_COUNT (3) */
    //     0x75, 0x01, /* REPORT_SIZE (1) */
    //     0x81, 0x02, /* INPUT (Data,Var,Abs) */
    //     0x95, 0x01, /* REPORT_COUNT (1) */
    //     0x75, 0x05, /* REPORT_SIZE (5) */
    //     0x81, 0x01, /* INPUT (Cnst,Var,Abs) */
    //     0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    //     0x09, 0x30, /* USAGE (X) */
    //     0x09, 0x31, /* USAGE (Y) */
    //     0x09, 0x38, /* USAGE (WHEEL) */
    //     0x15, 0x81, /* LOGICAL_MINIMUM (-127) */
    //     0x25, 0x7f, /* LOGICAL_MAXIMUM (127) */
    //     0x75, 0x08, /* REPORT_SIZE (8) */
    //     0x95, 0x03, /* REPORT_COUNT (3) */
    //     0x81, 0x06, /* INPUT (Data,Var,Rel) */
    //     0xc0, /* END_COLLECTION */
    //     0xc0, /* END_COLLECTION */
    //     0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    //     0x09, 0x06, /* USAGE (Keyboard) */
    //     0xa1, 0x01, /* COLLECTION (Application) */
    //     0x85, 0x02, /* REPORT_ID (2) */
    //     0x05, 0x08, /* USAGE_PAGE (Led) */
    //     0x19, 0x01, /* USAGE_MINIMUM (1) */
    //     0x29, 0x03, /* USAGE_MAXIMUM (3) */
    //     0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    //     0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    //     0x95, 0x03, /* REPORT_COUNT (3) */
    //     0x75, 0x01, /* REPORT_SIZE (1) */
    //     0x91, 0x02, /* Output (Data,Var,Abs) */
    //     0x95, 0x01, /* REPORT_COUNT (1) */
    //     0x75, 0x05, /* REPORT_SIZE (5) */
    //     0x91, 0x01, /* Output (Cnst,Var,Abs) */
    //     0xc0, /* END_COLLECTION */
    // ];

    let kb_data = vec![
        0x05, 0x01, // USAGE_PAGE (Generic Desktop)
        0x09, 0x06, // USAGE (Keyboard)		   /* 用途为键盘 */
        0xa1,
        0x01, // COLLECTION (Application) /* 表示应用结合，必须以END_COLLECTION来结束 */
        0x05, 0x07, //   USAGE_PAGE (Keyboard)  /* 用途页为按键 */
        0x19, 0xe0, //   USAGE_MINIMUM (Keyboard LeftControl)  /* 用途最小值 左Ctrl */
        0x29, 0xe7, //   USAGE_MAXIMUM (Keyboard Right GUI)	  /* 用途最大值 右GUI */
        0x15, 0x00, //   LOGICAL_MINIMUM (0)	   /* 逻辑最小值 0 */
        0x25, 0x01, //   LOGICAL_MAXIMUM (1)	   /* 逻辑最大值 1 */
        0x75,
        0x01, //   REPORT_SIZE (1)      /* 报告位大小(这个字段的宽度为1bit) */
        0x95,
        0x08, //   REPORT_COUNT (8) 	 /* 输入报告第一个字节(报告位大小 8bit) */
        0x81,
        0x02, //   INPUT (Data,Var,Abs) /* 报告为输入用 , 从左ctrl到右GUI 8bit刚好构成1个字节*/
        0x95, 0x01, //   REPORT_COUNT (1)	 /* 报告位数量  1个 */
        0x75,
        0x08, //   REPORT_SIZE (8)      /* 输入报告的第二给字节(报告位大小 8bit) */
        0x81,
        0x03, //   INPUT (Cnst,Var,Abs) /* 输入用的保留位，设备必须返回0 */
        0x95, 0x05, //   REPORT_COUNT (5)     /* 报告位数量 5个 */
        0x75, 0x01, //   REPORT_SIZE (1)		 /* 报告位大小，1bit */
        0x05, 0x08, //   USAGE_PAGE (LEDs)    /* 用途为LED */
        0x19, 0x01, //   USAGE_MINIMUM (Num Lock) /* 用途最小值 NUM Lock LED灯 */
        0x29, 0x05, //   USAGE_MAXIMUM (Kana)  /* 用途最大值 Kana 灯 */
        0x91, 0x02, //   OUTPUT (Data,Var,Abs) /* 输出用途，用于控制LED等 */
        0x95, 0x01, //   REPORT_COUNT (1)     /* 报告位数量 1个 */
        0x75, 0x03, //   REPORT_SIZE (3)      /* 报告位大小 3bit */
        0x91,
        0x03, //   OUTPUT (Cnst,Var,Abs)/* 用于字节补齐，跟前面5个bit进行补齐 */
        0x95,
        0x02, //0x06,                    //   REPORT_COUNT (6)  /* 报告位数量 6个*/
        0x75, 0x08, //   REPORT_SIZE (8)		 /* 报告位大小 8bit */
        0x15, 0x00, //   LOGICAL_MINIMUM (0)		 /* 逻辑最小值0 */
        0x25, 0xFF, //   LOGICAL_MAXIMUM (255)    /* 逻辑最大值255 */
        0x05, 0x07, //   USAGE_PAGE (Keyboard)    /* 用途页为按键 */
        0x19,
        0x00, //   USAGE_MINIMUM (Reserved (no event indicated)) /* 使用值最小为0 */
        0x29, 0x65, //   USAGE_MAXIMUM (Keyboard Application)		  /* 使用值最大为65 */
        0x81, 0x00, //   INPUT (Data,Ary,Abs)	 /* 输入用，变量，数组，绝对值 */
        0xc0,
    ];

    let create_params = CreateParams {
        name: "test-uhid-device".into(),
        phys: "".into(),
        uniq: "".into(),
        bus: Bus::USB,
        vendor: 0x15d9,
        product: 0x0a37,
        version: 0,
        country: 0,
        rd_data: kb_data,
    };

    let mut uhid_device = UHIDDevice::create(create_params).unwrap();

    let button_flags = 0;
    let mouse_abs_hor = 20;
    let mouse_abs_ver = 0;
    let wheel = 0;
    let data = vec![1, button_flags, mouse_abs_hor, mouse_abs_ver, wheel];
    let kb_data = vec![0, 7, 0, 5]; // b

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).unwrap();
        uhid_device.write(&kb_data).unwrap();
    }
}
