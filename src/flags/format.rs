enum_! {
    #[repr(u32)]
    /// Resource data formats, including fully-typed and typeless formats. A
    /// list of modifiers at the bottom of the page more fully describes each
    /// format type.
    ///
    /// ## Remarks
    ///
    /// ### Byte Order (LSB/MSB)
    /// Most formats have byte-aligned components, and the components are in
    /// C-array order (the least address comes first). For those formats that
    /// don't have power-of-2-aligned components, the first named component is
    /// in the least-significant bits.
    ///
    /// ### Portable Coding for Endian-Independence
    /// Rather than adjusting for whether a system uses big-endian or
    /// little-endian byte ordering, you should write portable code, as follows.
    ///
    /// ```
    /// # let mut something = [0.0; 4];
    /// // Format::R32G32B32A32Float
    /// let slice: &mut [f32] = &mut something;
    /// slice[0] = 1.0; // R
    /// slice[1] = 0.0; // G
    /// slice[2] = 0.0; // B
    /// slice[3] = 0.5; // A
    ///
    /// // Format::R10G10B10A2Unorm
    /// let value: u32 = 0x3ff | (0x1 << 30); // R=0x3ff, and A=0x1
    /// ```
    ///
    /// ### Restrictions and notes on formats
    ///
    /// 1. A resource declared with the R32G32B32 family of formats cannot be
    /// used simultaneously for vertex and texture data. That is, you may not
    /// create a buffer resource with the R32G32B32 family of formats that uses
    /// any of the following bind flags: [`VERTEX_BUFFER`][1],
    /// [`INDEX_BUFFER`][2], [`CONSTANT_BUFFER`][3], or [`STREAM_OUTPUT`][4].
    /// 2. R1Unorm is designed specifically for text filtering,
    /// and must be used with a format-specific, configurable 8x8 filter mode.
    /// When calling an HLSL sampling function using this format, the address
    /// offset parameter must be set to (0,0).
    /// 3. A resource using a sub-sampled format (such as [`R8G8_B8G8Unorm`][5])
    /// must have a size that is a multiple of 2 in the x dimension.
    /// 4. Format is not available in Direct3D 10 and Direct3D 10.1
    /// 5. These float formats have an implied 1 added to their mantissa. If
    /// the exponent is not 0, 1.0 is added to the mantissa before applying the
    /// exponent.
    /// 6. These float formats do not have an implied 1 added to their mantissa.
    /// 7. Denorm support: the 9, 10, 11 and 16 bit float formats support denorms.
    /// 8. No denorm support: the 32 and 64 bit float formats flush denorms to zero.
    ///
    /// The following topics provide lists of the formats that particular
    /// hardware [feature levels][6] support:
    ///
    /// * [DXGI Format Support for Direct3D Feature Level 12.1 Hardware][7]
    /// * [DXGI Format Support for Direct3D Feature Level 12.0 Hardware][8]
    /// * [DXGI Format Support for Direct3D Feature Level 11.1 Hardware][9]
    /// * [DXGI Format Support for Direct3D Feature Level 11.0 Hardware][10]
    /// * [Hardware Support for Direct3D 10Level9 Formats][11]
    /// * [Hardware Support for Direct3D 10.1 Formats][12]
    /// * [Hardware Support for Direct3D 10 Formats][13]
    ///
    /// ### Format Modifiers
    ///
    /// Each enumeration value contains a format modifier which describes
    /// the data type.
    ///
    /// <table>
    /// <tr>
    /// <td>Float</td>
    /// <td>A floating-point value; 32-bit floating-point formats use IEEE 754
    /// single-precision (s23e8 format): sign bit, 8-bit biased (127) exponent,
    /// and 23-bit mantissa. 16-bit floating-point formats use half-precision
    /// (s10e5 format): sign bit, 5-bit biased (15) exponent, and 10-bit
    /// mantissa.</td>
    /// </tr>
    /// <tr>
    /// <td>Sint</td>
    /// <td>Two's complement signed integer. For example, a 3-bit Sint
    /// represents the values -4, -3, -2, -1, 0, 1, 2, 3.</td>
    /// </tr>
    /// <tr>
    /// <td>Snorm</td>
    /// <td>Signed normalized integer; which is interpreted in a resource as a
    /// signed integer, and is interpreted in a shader as a signed normalized
    /// floating-point value in the range `[-1, 1]`. For an 2's complement
    /// number, the maximum value is 1.0f (a 5-bit value `01111` maps to `1.0`),
    /// and the minimum value is -1.0f (a 5-bit value `10000` maps to `-1.0`).
    /// In addition, the second-minimum number maps to -1.0f (a 5-bit value
    /// `10001` maps to `-1.0`). The resulting integer representations are
    /// evenly spaced floating-point values in the range `(-1.0...0.0)`, and
    /// also a complementary set of representations for numbers in the range
    /// `(0.0f...1.0f)`.</td>
    /// </tr>
    /// <tr>
    /// <td>Srgb</td>
    /// <td>Standard RGB data, which roughly displays colors in a linear ramp of
    /// luminosity levels such that an average observer, under average viewing
    /// conditions, can view them on an average display.
    ///
    /// All 0's maps to 0.0f, and all 1's maps to 1.0f. The sequence of unsigned
    /// integer encodings between all 0's and all 1's represent a nonlinear
    /// progression in the floating-point interpretation of the numbers between
    /// 0.0f to 1.0f. For more detail, see the SRGB color standard, IEC
    /// 61996-2-1, at IEC (International Electrotechnical Commission).</td>
    /// </tr>
    /// <tr>
    /// <td>Typeless</td>
    /// <td>Typeless data, with a defined number of bits. Typeless formats are
    /// designed for creating typeless resources; that is, a resource whose size
    /// is known, but whose data type is not yet fully defined. When a typeless
    /// resource is bound to a shader, the application or shader must resolve
    /// the format type (which must match the number of bits per component in
    /// the typeless format).
    ///
    /// A typeless format contains one or more subformats; each subformat
    /// resolves the data type. For example, in the R32G32B32 group, which
    /// defines types for three-component 96-bit data, there is one typeless
    /// format and three fully typed subformats.</td>
    /// </tr>
    /// <tr>
    /// <td>Uint</td>
    /// <td>Unsigned integer. For instance, a 3-bit UINT represents the values
    /// 0, 1, 2, 3, 4, 5, 6, 7.</td>
    /// </tr>
    /// <tr>
    /// <td>Unorm</td>
    /// <td>Unsigned normalized integer; which is interpreted in a resource as
    /// an unsigned integer, and is interpreted in a shader as an unsigned
    /// normalized floating-point value in the range [0, 1]. All 0's maps to
    /// 0.0f, and all 1's maps to 1.0f. A sequence of evenly spaced
    /// floating-point values from 0.0f to 1.0f are represented. For instance,
    /// a 2-bit UNORM represents 0.0f, 1/3, 2/3, and 1.0f.</td>
    /// </tr>
    /// <tr>
    /// <td>SharedExponent</td>
    /// <td>A shared exponent. All the floating point representations in the
    /// format share the one exponent.</td>
    /// </tr>
    /// </table>
    ///
    /// ### New Resource Formats
    ///
    /// Direct3D 10 offers new data compression formats for compressing
    /// high-dynamic range (HDR) lighting data, normal maps and heightfields
    /// to a fraction of their original size. These compression types include:
    ///
    /// * Shared-Exponent high-dynamic range (HDR) format (RGBE)
    /// * New Block-Compressed 1-2 channel Unorm/Snorm formats
    ///
    /// The block compression formats can be used for any of the 2D or 3D
    /// texture types (Texture2D, Texture2DArray, Texture3D, or TextureCube)
    /// including mipmap surfaces. The block compression techniques require
    /// texture dimensions to be a multiple of 4 (since the implementation
    /// compresses on blocks of 4x4 texels). In the texture sampler, compressed
    /// formats are always decompressed before texture filtering.
    ///
    /// [1]: struct.BindFlags.html#VERTEX_BUFFER
    /// [2]: struct.BindFlags.html#INDEX_BUFFER
    /// [3]: struct.BindFlags.html#CONSTANT_BUFFER
    /// [4]: struct.BindFlags.html#STREAM_OUTPUT
    /// [5]: #variant.R8G8_B8G8Unorm
    /// [6]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476876(v=vs.85).aspx#Overview
    /// [7]: https://msdn.microsoft.com/en-us/library/windows/desktop/mt426648(v=vs.85).aspx
    /// [8]: https://msdn.microsoft.com/en-us/library/windows/desktop/mt426647(v=vs.85).aspx
    /// [9]: https://msdn.microsoft.com/en-us/library/windows/desktop/mt427456(v=vs.85).aspx
    /// [10]: https://msdn.microsoft.com/en-us/library/windows/desktop/mt427455(v=vs.85).aspx
    /// [11]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff471324(v=vs.85).aspx
    /// [12]: https://msdn.microsoft.com/en-us/library/windows/desktop/cc627091(v=vs.85).aspx
    /// [13]: https://msdn.microsoft.com/en-us/library/windows/desktop/cc627090(v=vs.85).aspx
    pub enum Format {
        /// The format is not known.
        Unknown = 0,
        /// A four-component, 128-bit typeless format that supports 32 bits per
        /// channel including alpha.
        /// <sup>[1](#restrictions-and-notes-on-formats)</sup>
        R32G32B32A32Typeless = 1,

        /// A four-component, 128-bit floating-point format that supports 32
        /// bits per channel including alpha.
        /// <sup>[1,5,8](#restrictions-and-notes-on-formats)</sup>
        R32G32B32A32Float = 2,

        /// A four-component, 128-bit unsigned-integer format that supports 32
        /// bits per channel including alpha.
        /// <sup>[1](#restrictions-and-notes-on-formats)</sup>
        R32G32B32A32Uint = 3,

        /// A four-component, 128-bit signed-integer format that supports 32
        /// bits per channel including alpha.
        /// <sup>[1](#restrictions-and-notes-on-formats)</sup>
        R32G32B32A32Sint = 4,

        /// A three-component, 96-bit typeless format that supports 32 bits per
        /// color channel.
        R32G32B32Typeless = 5,

        /// A three-component, 96-bit floating-point format that supports 32
        /// bits per color channel.
        /// <sup>[5,8](#restrictions-and-notes-on-formats)</sup>
        R32G32B32Float = 6,

        /// A three-component, 96-bit unsigned-integer format that supports 32
        /// bits per color channel.
        R32G32B32Uint = 7,

        /// A three-component, 96-bit signed-integer format that supports 32
        /// bits per color channel.
        R32G32B32Sint = 8,

        /// A four-component, 64-bit typeless format that supports 16 bits per
        /// channel including alpha.
        R16G16B16A16Typeless = 9,

        /// A four-component, 64-bit floating-point format that supports 16
        /// bits per channel including alpha.
        /// <sup>[5,7](#restrictions-and-notes-on-formats)</sup>
        R16G16B16A16Float = 10,

        /// A four-component, 64-bit unsigned-normalized-integer format that
        /// supports 16 bits per channel including alpha.
        R16G16B16A16Unorm = 11,

        /// A four-component, 64-bit unsigned-integer format that supports 16
        /// bits per channel including alpha.
        R16G16B16A16Uint = 12,

        /// A four-component, 64-bit signed-normalized-integer format that
        /// supports 16 bits per channel including alpha.
        R16G16B16A16Snorm = 13,

        /// A four-component, 64-bit signed-integer format that supports 16
        /// bits per channel including alpha.
        R16G16B16A16Sint = 14,

        /// A two-component, 64-bit typeless format that supports 32 bits for
        /// the red channel and 32 bits for the green channel.
        R32G32Typeless = 15,

        /// A two-component, 64-bit floating-point format that supports 32 bits
        /// for the red channel and 32 bits for the green channel.
        /// <sup>[5,8](#restrictions-and-notes-on-formats)</sup>
        R32G32Float = 16,

        /// A two-component, 64-bit unsigned-integer format that supports 32
        /// bits for the red channel and 32 bits for the green channel.
        R32G32Uint = 17,

        /// A two-component, 64-bit signed-integer format that supports 32 bits
        /// for the red channel and 32 bits for the green channel.
        R32G32Sint = 18,

        /// A two-component, 64-bit typeless format that supports 32 bits for
        /// the red channel, 8 bits for the green channel, and 24 bits are unused.
        R32G8X24Typeless = 19,

        /// A 32-bit floating-point component, and two unsigned-integer
        /// components (with an additional 32 bits). This format supports
        /// 32-bit depth, 8-bit stencil, and 24 bits are unused.
        /// <sup>[5](#restrictions-and-notes-on-formats)</sup>
        D32FloatS8X24Uint = 20,

        /// A 32-bit floating-point component, and two typeless components
        /// (with an additional 32 bits). This format supports 32-bit red
        /// channel, 8 bits are unused, and 24 bits are unused.
        /// <sup>[5](#restrictions-and-notes-on-formats)</sup>
        R32FloatX8X24Typeless = 21,

        /// A 32-bit typeless component, and two unsigned-integer components
        /// (with an additional 32 bits). This format has 32 bits unused, 8
        /// bits for green channel, and 24 bits are unused.
        X32TypelessG8X24Uint = 22,

        /// A four-component, 32-bit typeless format that supports 10 bits for
        /// each color and 2 bits for alpha.
        R10G10B10A2Typeless = 23,

        /// A four-component, 32-bit unsigned-normalized-integer format that
        /// supports 10 bits for each color and 2 bits for alpha.
        R10G10B10A2Unorm = 24,

        /// A four-component, 32-bit unsigned-integer format that supports 10
        /// bits for each color and 2 bits for alpha.
        R10G10B10A2Uint = 25,

        /// Three partial-precision floating-point numbers encoded into a single
        /// 32-bit value (a variant of s10e5, which is sign bit, 10-bit mantissa,
        /// and 5-bit biased (15) exponent). There are no sign bits, and there
        /// is a 5-bit biased (15) exponent for each channel, 6-bit mantissa for
        /// R and G, and a 5-bit mantissa for B, as shown in the following
        /// illustration.
        /// <sup>[5,7](#restrictions-and-notes-on-formats)</sup>
        ///
        /// ![Illustration of the bits in three partial-precision floating point numbers][1]
        ///
        /// [1]: https://msdn.microsoft.com/dynimg/IC534131.png
        R11G11B10Float = 26,

        /// A four-component, 32-bit typeless format that supports 8 bits per
        /// channel including alpha.
        R8G8B8A8Typeless = 27,

        /// A four-component, 32-bit unsigned-normalized-integer format that
        /// supports 8 bits per channel including alpha.
        R8G8B8A8Unorm = 28,

        /// A four-component, 32-bit unsigned-normalized integer sRGB format
        /// that supports 8 bits per channel including alpha.
        R8G8B8A8UnormSrgb = 29,

        /// A four-component, 32-bit unsigned-integer format that supports 8
        /// bits per channel including alpha.
        R8G8B8A8Uint = 30,

        /// A four-component, 32-bit signed-normalized-integer format that
        /// supports 8 bits per channel including alpha.
        R8G8B8A8Snorm = 31,

        /// A four-component, 32-bit signed-integer format that supports 8
        /// bits per channel including alpha.
        R8G8B8A8Sint = 32,

        /// A two-component, 32-bit typeless format that supports 16 bits for
        /// the red channel and 16 bits for the green channel.
        R16G16Typeless = 33,

        /// A two-component, 32-bit floating-point format that supports 16
        /// bits for the red channel and 16 bits for the green channel.
        /// <sup>[5,7](#restrictions-and-notes-on-formats)</sup>
        R16G16Float = 34,

        /// A two-component, 32-bit unsigned-normalized-integer format that
        /// supports 16 bits each for the green and red channels.
        R16G16Unorm = 35,

        /// A two-component, 32-bit unsigned-integer format that supports 16
        /// bits for the red channel and 16 bits for the green channel.
        R16G16Uint = 36,

        /// A two-component, 32-bit signed-normalized-integer format that
        /// supports 16 bits for the red channel and 16 bits for the green
        /// channel.
        R16G16Snorm = 37,

        /// A two-component, 32-bit signed-integer format that supports 16
        /// bits for the red channel and 16 bits for the green channel.
        R16G16Sint = 38,

        /// A single-component, 32-bit typeless format that supports 32 bits
        /// for the red channel.
        R32Typeless = 39,

        /// A single-component, 32-bit floating-point format that supports 32
        /// bits for depth.
        /// <sup>[5,8](#restrictions-and-notes-on-formats)</sup>
        D32Float = 40,

        /// A single-component, 32-bit floating-point format that supports 32
        /// bits for the red channel.
        /// <sup>[5,8](#restrictions-and-notes-on-formats)</sup>
        R32Float = 41,

        /// A single-component, 32-bit unsigned-integer format that supports 32
        /// bits for the red channel.
        R32Uint = 42,

        /// A single-component, 32-bit signed-integer format that supports 32
        /// bits for the red channel.
        R32Sint = 43,

        /// A two-component, 32-bit typeless format that supports 24 bits for
        /// the red channel and 8 bits for the green channel.
        R24G8Typeless = 44,

        /// A 32-bit z-buffer format that supports 24 bits for depth and 8 bits
        /// for stencil.
        D24UnormS8Uint = 45,

        /// A 32-bit format, that contains a 24 bit, single-component,
        /// unsigned-normalized integer, with an additional typeless 8 bits.
        /// This format has 24 bits red channel and 8 bits unused.
        R24UnormX8Typeless = 46,

        /// A 32-bit format, that contains a 24 bit, single-component, typeless
        /// format, with an additional 8 bit unsigned integer component. This
        /// format has 24 bits unused and 8 bits green channel.
        X24TypelessG8Uint = 47,

        /// A two-component, 16-bit typeless format that supports 8 bits for
        /// the red channel and 8 bits for the green channel.
        R8G8Typeless = 48,

        /// A two-component, 16-bit unsigned-normalized-integer format that
        /// supports 8 bits for the red channel and 8 bits for the green
        /// channel.
        R8G8Unorm = 49,

        /// A two-component, 16-bit unsigned-integer format that supports 8
        /// bits for the red channel and 8 bits for the green channel.
        R8G8Uint = 50,

        /// A two-component, 16-bit signed-normalized-integer format that
        /// supports 8 bits for the red channel and 8 bits for the green
        /// channel.
        R8G8Snorm = 51,

        /// A two-component, 16-bit signed-integer format that supports 8 bits
        /// for the red channel and 8 bits for the green channel.
        R8G8Sint = 52,

        /// A single-component, 16-bit typeless format that supports 16 bits
        /// for the red channel.
        R16Typeless = 53,

        /// A single-component, 16-bit floating-point format that supports 16
        /// bits for the red channel.
        /// <sup>[5,7](#restrictions-and-notes-on-formats)</sup>
        R16Float = 54,

        /// A single-component, 16-bit unsigned-normalized-integer format that
        /// supports 16 bits for depth.
        D16Unorm = 55,

        /// A single-component, 16-bit unsigned-normalized-integer format that
        /// supports 16 bits for the red channel.
        R16Unorm = 56,

        /// A single-component, 16-bit unsigned-integer format that supports
        /// 16 bits for the red channel.
        R16Uint = 57,

        /// A single-component, 16-bit signed-normalized-integer format that
        /// supports 16 bits for the red channel.
        R16Snorm = 58,

        /// A single-component, 16-bit signed-integer format that supports 16
        /// bits for the red channel.
        R16Sint = 59,

        /// A single-component, 8-bit typeless format that supports 8 bits for
        /// the red channel.
        R8Typeless = 60,

        /// A single-component, 8-bit unsigned-normalized-integer format that
        /// supports 8 bits for the red channel.
        R8Unorm = 61,

        /// A single-component, 8-bit unsigned-integer format that supports 8
        /// bits for the red channel.
        R8Uint = 62,

        /// A single-component, 8-bit signed-normalized-integer format that
        /// supports 8 bits for the red channel.
        R8Snorm = 63,

        /// A single-component, 8-bit signed-integer format that supports 8
        /// bits for the red channel.
        R8Sint = 64,

        /// A single-component, 8-bit unsigned-normalized-integer format for
        /// alpha only.
        A8Unorm = 65,

        /// A single-component, 1-bit unsigned-normalized integer format that
        /// supports 1 bit for the red channel.
        /// <sup>[2](#restrictions-and-notes-on-formats)</sup>
        R1Unorm = 66,

        /// Three partial-precision floating-point numbers encoded into a
        /// single 32-bit value all sharing the same 5-bit exponent (variant
        /// of s10e5, which is sign bit, 10-bit mantissa, and 5-bit biased
        /// (15) exponent). There is no sign bit, and there is a shared 5-bit
        /// biased (15) exponent and a 9-bit mantissa for each channel, as
        /// shown in the following illustration.
        /// <sup>[2,6,7](#restrictions-and-notes-on-formats)</sup>
        ///
        /// ![Illustration of the bits in the three partial-precision floating-point numbers][1]
        ///
        /// [1]: https://msdn.microsoft.com/dynimg/IC534132.png
        R9G9B9E5SharedExponent = 67,

        /// A four-component, 32-bit unsigned-normalized-integer format. This
        /// packed RGB format is analogous to the UYVY format. Each 32-bit
        /// block describes a pair of pixels: (R8, G8, B8) and (R8, G8, B8)
        /// where the R8/B8 values are repeated, and the G8 values are unique
        /// to each pixel.
        /// <sup>[3](#restrictions-and-notes-on-formats)</sup>
        ///
        /// Width must be even.
        #[allow(non_camel_case_types)]
        R8G8_B8G8Unorm = 68,

        /// A four-component, 32-bit unsigned-normalized-integer format. This
        /// packed RGB format is analogous to the YUY2 format. Each 32-bit
        /// block describes a pair of pixels: (R8, G8, B8) and (R8, G8, B8)
        /// where the R8/B8 values are repeated, and the G8 values are unique
        /// to each pixel.
        /// <sup>[3](#restrictions-and-notes-on-formats)</sup>
        ///
        /// Width must be even.
        #[allow(non_camel_case_types)]
        G8R8_G8B8Unorm = 69,

        /// Four-component typeless block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc1Typeless = 70,

        /// Four-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc1Unorm = 71,

        /// Four-component block-compression format for sRGB data.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc1UnormSrgb = 72,

        /// Four-component typeless block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc2Typeless = 73,

        /// Four-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc2Unorm = 74,

        /// Four-component block-compression format for sRGB data.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc2UnormSrgb = 75,

        /// Four-component typeless block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc3Typeless = 76,

        /// Four-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc3Unorm = 77,

        /// Four-component block-compression format for sRGB data.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc3UnormSrgb = 78,

        /// One-component typeless block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc4Typeless = 79,

        /// One-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc4Unorm = 80,

        /// One-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc4Snorm = 81,

        /// Two-component typeless block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc5Typeless = 82,

        /// Two-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc5Unorm = 83,

        /// Two-component block-compression format.
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc5Snorm = 84,

        /// A three-component, 16-bit unsigned-normalized-integer format that
        /// supports 5 bits for blue, 6 bits for green, and 5 bits for red.
        ///
        /// **Direct3D 10 through Direct3D 11:** This value is defined for
        /// DXGI. However, Direct3D 10, 10.1, or 11 devices do not support
        /// this format.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        B5G6R5Unorm = 85,

        /// A four-component, 16-bit unsigned-normalized-integer format that
        /// supports 5 bits for each color channel and 1-bit alpha.
        ///
        /// **Direct3D 10 through Direct3D 11:** This value is defined for
        /// DXGI. However, Direct3D 10, 10.1, or 11 devices do not support
        /// this format.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        B5G5R5A1Unorm = 86,

        /// A four-component, 32-bit unsigned-normalized-integer format that
        /// supports 8 bits for each color channel and 8-bit alpha.
        B8G8R8A8Unorm = 87,

        /// A four-component, 32-bit unsigned-normalized-integer format that
        /// supports 8 bits for each color channel and 8 bits unused.
        B8G8R8X8Unorm = 88,

        /// A four-component, 32-bit 2.8-biased fixed-point format that
        /// supports 10 bits for each color channel and 2-bit alpha.
        R10G10B10XrBiasA2Unorm = 89,

        /// A four-component, 32-bit typeless format that supports 8 bits
        /// for each channel including alpha.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        B8G8R8A8Typeless = 90,

        /// A four-component, 32-bit unsigned-normalized standard RGB format
        /// that supports 8 bits for each channel including alpha.
        /// <sup>[3](#restrictions-and-notes-on-formats)</sup>
        B8G8R8A8UnormSrgb = 91,

        /// A four-component, 32-bit typeless format that supports 8 bits for
        /// each color channel, and 8 bits are unused.
        /// <sup>[3](#restrictions-and-notes-on-formats)</sup>
        B8G8R8X8Typeless = 92,

        /// A four-component, 32-bit unsigned-normalized standard RGB format
        /// that supports 8 bits for each color channel, and 8 bits are unused.
        /// <sup>[3](#restrictions-and-notes-on-formats)</sup>
        B8G8R8X8UnormSrgb = 93,

        /// A typeless block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc6hTypeless = 94,

        /// A block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        /// <sup>[5](#restrictions-and-notes-on-formats)</sup>
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc6hUf16 = 95,

        /// A block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        /// <sup>[5](#restrictions-and-notes-on-formats)</sup>
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc6hSf16 = 96,

        /// A typeless block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc7Typeless = 97,

        /// A block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc7Unorm = 98,

        /// A block-compression format.
        /// <sup>[4](#restrictions-and-notes-on-formats)</sup>
        /// For information about block-compression formats, see
        /// [Texture Block Compression in Direct3D 11.][1]
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh308955(v=vs.85).aspx
        Bc7UnormSrgb = 99,

        /// Most common YUV 4:4:4 video resource format. Valid view formats
        /// for this video resource format are
        /// [R8G8B8A8Unorm](#variant.R8G8B8A8Unorm) and
        /// [R8G8B8A8Uint](#variant.R8G8B8A8Uint). For UAVs, an additional
        /// valid view format is [R32Uint](#variant.R32Uint). By using
        /// [R32Uint](#variant.R32Uint) for UAVs, you can both read and write
        /// as opposed to just write for [R8G8B8A8Unorm](#variant.R8G8B8A8Unorm)
        /// and [R8G8B8A8Uint](#variant.R8G8B8A8Uint). Supported view types are
        /// SRV, RTV, and UAV. One view provides a straightforward mapping of
        /// the entire surface. The mapping to the view channel is V->R8, U->G8,
        /// Y->B8, and A->A8.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Ayuv = 100,

        /// 10-bit per channel packed YUV 4:4:4 video resource format. Valid
        /// view formats for this video resource format are
        /// [R10G10B10A2Unorm](#variant.R10G10B10A2Unorm) and
        /// [R10G10B10A2Uint](#variant.R10G10B10A2Uint). For UAVs, an additional
        /// valid view format is [R32Uint](#variant.R32Uint). By using
        /// [R32Uint](#variant.R32Uint) for UAVs, you can both read and write as
        /// opposed to just write for [R10G10B10A2Unorm](#variant.R10G10B10A2Unorm)
        /// and [R10G10B10A2Uint](#variant.R10G10B10A2Uint). Supported view types
        /// are SRV and UAV. One view provides a straightforward mapping of the
        /// entire surface. The mapping to the view channel is U->R10, Y->G10,
        /// V->B10, and A->A2.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Y410 = 101,

        /// 16-bit per channel packed YUV 4:4:4 video resource format. Valid
        /// view formats for this video resource format are
        /// [R16G16B16A16Unorm](#variant.R16G16B16A16Unorm) and
        /// [R16G16B16A16Uint](#variant.R16G16B16A16Uint). Supported view types
        /// are SRV and UAV. One view provides a straightforward mapping of the
        /// entire surface. The mapping to the view channel is U->R16, Y->G16,
        /// V->B16, and A->A16.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Y416 = 102,

        /// Most common YUV 4:2:0 video resource format. Valid luminance data
        /// view formats for this video resource format are
        /// [R8Unorm](#variant.R8Unorm) and [R8Uint](#variant.R8Uint). Valid
        /// chrominance data view formats (width and height are each 1/2 of
        /// luminance view) for this video resource format are
        /// [R8G8Unorm](#variant.R8G8Unorm) and [R8G8Uint](#variant.R8G8Uint).
        /// Supported view types are SRV, RTV, and UAV. For luminance data view,
        /// the mapping to the view channel is Y->R8. For chrominance data view,
        /// the mapping to the view channel is U->R8 and V->G8.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width and height must be even. Direct3D 11 staging resources and
        /// initData parameters for this format use
        /// `(rowPitch * (height + (height / 2)))` bytes. The first
        /// `(SysMemPitch * height)` bytes are the Y plane, the remaining
        /// `(SysMemPitch * (height / 2))` bytes are the UV plane.
        ///
        /// An app using the YUY 4:2:0 formats must map the luma (Y) plane
        /// separately from the chroma (UV) planes. Developers do this by
        /// calling [ID3D12Device::CreateShaderResourceView][2] twice for the
        /// same texture and passing in 1-channel and 2-channel formats.
        /// Passing in a 1-channel format compatible with the Y plane maps
        /// only the Y plane. Passing in a 2-channel format compatible with
        /// the UV planes (together) maps only the U and V planes as a single
        /// resource view.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        /// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/dn788672(v=vs.85).aspx
        Nv12 = 103,

        /// 10-bit per channel planar YUV 4:2:0 video resource format. Valid
        /// luminance data view formats for this video resource format are
        /// [R16Unorm](#variant.R16Unorm) and [R16Uint](#variant.R16Uint).
        /// The runtime does not enforce whether the lowest 6 bits are 0
        /// (given that this video resource format is a 10-bit format that
        /// uses 16 bits). If required, application shader code would have to
        /// enforce this manually. From the runtime's point of view,
        /// [P010](#variant.P010) is no different than [P016](#variant.P016).
        /// Valid chrominance data view formats (width and height are each 1/2
        /// of luminance view) for this video resource format are
        /// [R16G16Unorm](#variant.R16G16Unorm) and
        /// [R16G16Uint](#variant.R16G16Uint). For UAVs, an additional valid
        /// chrominance data view format is [R32Uint](#variant.R32Uint). By
        /// using [R32Uint](#variant.R32Uint) for UAVs, you can both read and
        /// write as opposed to just write for
        /// [R16G16Unorm](#variant.R16G16Unorm) and
        /// [R16G16Uint](#variant.R16G16Uint). Supported view types are SRV,
        /// RTV, and UAV. For luminance data view, the mapping to the view
        /// channel is Y->R16. For chrominance data view, the mapping to the
        /// view channel is U->R16 and V->G16.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width and height must be even. Direct3D 11 staging resources and
        /// initData parameters for this format use
        /// `(rowPitch * (height + (height / 2)))` bytes. The first
        /// `(SysMemPitch * height)` bytes are the Y plane, the remaining
        /// `(SysMemPitch * (height / 2))` bytes are the UV plane.
        ///
        /// An app using the YUY 4:2:0 formats must map the luma (Y) plane
        /// separately from the chroma (UV) planes. Developers do this by
        /// calling [ID3D12Device::CreateShaderResourceView][2] twice for the
        /// same texture and passing in 1-channel and 2-channel formats.
        /// Passing in a 1-channel format compatible with the Y plane maps
        /// only the Y plane. Passing in a 2-channel format compatible with
        /// the UV planes (together) maps only the U and V planes as a single
        /// resource view.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        /// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/dn788672(v=vs.85).aspx
        P010 = 104,

        /// 16-bit per channel planar YUV 4:2:0 video resource format. Valid
        /// luminance data view formats for this video resource format are
        /// [R16Unorm](#variant.R16Unorm) and [R16Uint](#variant.R16Uint).
        /// Valid chrominance data view formats (width and height are each 1/2
        /// of luminance view) for this video resource format are
        /// [R16G16Unorm](#variant.R16G16Unorm) and
        /// [R16G16Uint](#variant.R16G16Uint). For UAVs, an additional valid
        /// chrominance data view format is [R32Uint](#variant.R32Uint). By
        /// using [R32Uint](#variant.R32Uint) for UAVs, you can both read and
        /// write as opposed to just write for
        /// [R16G16Unorm](#variant.R16G16Unorm) and
        /// [R16G16Uint](#variant.R16G16Uint). Supported view types are SRV,
        /// RTV, and UAV. For luminance data view, the mapping to the view
        /// channel is Y->R16. For chrominance data view, the mapping to the
        /// view channel is U->R16 and V->G16.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width and height must be even. Direct3D 11 staging resources and
        /// initData parameters for this format use
        /// `(rowPitch * (height + (height / 2)))` bytes. The first
        /// `(SysMemPitch * height)` bytes are the Y plane, the remaining
        /// `(SysMemPitch * (height / 2))` bytes are the UV plane.
        ///
        /// An app using the YUY 4:2:0 formats must map the luma (Y) plane
        /// separately from the chroma (UV) planes. Developers do this by
        /// calling [ID3D12Device::CreateShaderResourceView][2] twice for the
        /// same texture and passing in 1-channel and 2-channel formats.
        /// Passing in a 1-channel format compatible with the Y plane maps
        /// only the Y plane. Passing in a 2-channel format compatible with
        /// the UV planes (together) maps only the U and V planes as a single
        /// resource view.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        /// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/dn788672(v=vs.85).aspx
        P016 = 105,

        /// 8-bit per channel planar YUV 4:2:0 video resource format. This
        /// format is subsampled where each pixel has its own Y value, but
        /// each 2x2 pixel block shares a single U and V value. The runtime
        /// requires that the width and height of all resources that are
        /// created with this format are multiples of 2. The runtime also
        /// requires that the left, right, top, and bottom members of any
        /// `RECT` that are used for this format are multiples of 2. This
        /// format differs from [Nv12](#variant.Nv12) in that the layout of
        /// the data within the resource is completely opaque to applications.
        /// Applications cannot use the CPU to map the resource and then
        /// access the data within the resource. You cannot use shaders with
        /// this format. Because of this behavior, legacy hardware that
        /// supports a non-NV12 4:2:0 layout (for example, YV12, and so on)
        /// can be used. Also, new hardware that has a 4:2:0 implementation
        /// better than NV12 can be used when the application does not need
        /// the data to be in a standard layout.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width and height must be even. Direct3D 11 staging resources and
        /// initData parameters for this format use
        /// `(rowPitch * (height + (height / 2)))` bytes. The first
        /// `(SysMemPitch * height)` bytes are the Y plane, the remaining
        /// `(SysMemPitch * (height / 2))` bytes are the UV plane.
        ///
        /// An app using the YUY 4:2:0 formats must map the luma (Y) plane
        /// separately from the chroma (UV) planes. Developers do this by
        /// calling [ID3D12Device::CreateShaderResourceView][2] twice for the
        /// same texture and passing in 1-channel and 2-channel formats.
        /// Passing in a 1-channel format compatible with the Y plane maps
        /// only the Y plane. Passing in a 2-channel format compatible with
        /// the UV planes (together) maps only the U and V planes as a single
        /// resource view.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        /// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/dn788672(v=vs.85).aspx
        Yuv420Opaque = 106,

        /// Most common YUV 4:2:2 video resource format. Valid view formats
        /// for this video resource format are
        /// [R8G8B8A8Unorm](#variant.R8G8B8A8Unorm) and
        /// [R8G8B8A8Uint](#variant.R8G8B8A8Uint). For UAVs, an additional
        /// valid view format is [R32Uint](#variant.R32Uint). By using
        /// [R32Uint](#variant.R32Uint) for UAVs, you can both read and write
        /// as opposed to just write for
        /// [R8G8B8A8Unorm](#variant.R8G8B8A8Unorm) and
        /// [R8G8B8A8Uint](#variant.R8G8B8A8Uint). Supported view types are
        /// SRV and UAV. One view provides a straightforward mapping of the
        /// entire surface. The mapping to the view channel is Y0->R8, U0->G8,
        /// Y1->B8, and V0->A8.
        ///
        /// A unique valid view format for this video resource format is
        /// [R8G8_B8G8Unorm](#variant.R8G8_B8G8Unorm). With this view format,
        /// the width of the view appears to be twice what the
        /// [R8G8B8A8Unorm](#variant.R8G8B8A8Unorm) or
        /// [R8G8B8A8Uint](#variant.R8G8B8A8Uint) view would be when hardware
        /// reconstructs RGBA automatically on read and before filtering. This
        /// Direct3D hardware behavior is legacy and is likely not useful any
        /// more. With this view format, the mapping to the view channel is
        /// Y0->R8, U0->G8[0], Y1->B8, and V0->G8[1].
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width must be even.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Yuy2 = 107,

        /// 10-bit per channel packed YUV 4:2:2 video resource format. Valid
        /// view formats for this video resource format are
        /// [R16G16B16A16Unorm](#variant.R16G16B16A16Unorm) and
        /// [R16G16B16A16Uint](#variant.R16G16B16A16Uint). The runtime does
        /// not enforce whether the lowest 6 bits are 0 (given that this video
        /// resource format is a 10-bit format that uses 16 bits). If required,
        /// application shader code would have to enforce this manually. From
        /// the runtime's point of view, [Y210](#variant.Y210) is no different
        /// than [Y216](#variant.Y216). Supported view types are SRV and UAV.
        /// One view provides a straightforward mapping of the entire surface.
        /// The mapping to the view channel is Y0->R16, U->G16, Y1->B16, and
        /// V->A16.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width must be even.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Y210 = 108,

        /// 16-bit per channel packed YUV 4:2:2 video resource format. Valid
        /// view formats for this video resource format are
        /// [R16G16B16A16Unorm](#variant.R16G16B16A16Unorm) and
        /// [R16G16B16A16Uint](#variant.R16G16B16A16Uint). Supported view types
        /// are SRV and UAV. One view provides a straightforward mapping of the
        /// entire surface. The mapping to the view channel is Y0->R16, U->G16,
        /// Y1->B16, and V->A16.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width must be even.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Y216 = 109,

        /// Most common planar YUV 4:1:1 video resource format. Valid luminance
        /// data view formats for this video resource format are
        /// [R8Unorm](#variant.R8Unorm) and [R8Uint](#variant.R8Uint). Valid
        /// chrominance data view formats (width and height are each 1/4 of
        /// luminance view) for this video resource format are
        /// [R8G8Unorm](#variant.R8G8Unorm) and [R8G8Uint](#variant.R8G8Uint).
        /// Supported view types are SRV, RTV, and UAV. For luminance data
        /// view, the mapping to the view channel is Y->R8. For chrominance
        /// data view, the mapping to the view channel is U->R8 and V->G8.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// Width must be a multiple of 4. Direct3D11 staging resources and
        /// initData parameters for this format use `(rowPitch * height * 2)`
        /// bytes. The first `(SysMemPitch * height)` bytes are the Y plane,
        /// the next `((SysMemPitch / 2) * height)` bytes are the UV plane,
        /// and the remainder is padding.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Nv11 = 110,

        /// 4-bit palletized YUV format that is commonly used for DVD subpicture.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Ai44 = 111,

        /// 4-bit palletized YUV format that is commonly used for DVD subpicture.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        Ia44 = 112,

        /// 8-bit palletized format that is used for palletized RGB data when
        /// the processor processes ISDB-T data and for palletized YUV data
        /// when the processor processes BluRay data.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        P8 = 113,

        /// 8-bit palletized format with 8 bits of alpha that is used for
        /// palletized YUV data when the processor processes BluRay data.
        ///
        /// For more info about YUV formats for video rendering, see
        /// [Recommended 8-Bit YUV Formats for Video Rendering.][1]
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd206750(v=vs.85).aspx
        A8P8 = 114,

        /// A four-component, 16-bit unsigned-normalized integer format that
        /// supports 4 bits for each channel including alpha.
        ///
        /// **Direct3D 11.1:** This value is not supported until Windows 8.
        B4G4R4A4Unorm = 115,

        /// A video format; an 8-bit version of a hybrid planar 4:2:2 format.
        P208 = 130,

        /// An 8 bit YCbCrA 4:4 rendering format.
        V208 = 131,

        /// An 8 bit YCbCrA 4:4:4:4 rendering format.
        V408 = 132,
    }
}

impl Format {
    /// Gets the number of bytes per pixel in a texture with this format.
    /// Returns `0` if the format does not have a simple answer to this
    /// question.
    pub fn pixel_size(self) -> usize {
        use self::Format::*;
        match self {
            Unknown => 0,
            R32G32B32A32Typeless => 16,
            R32G32B32A32Float => 16,
            R32G32B32A32Uint => 16,
            R32G32B32A32Sint => 16,
            R32G32B32Typeless => 12,
            R32G32B32Float => 12,
            R32G32B32Uint => 12,
            R32G32B32Sint => 12,
            R16G16B16A16Typeless => 8,
            R16G16B16A16Float => 8,
            R16G16B16A16Unorm => 8,
            R16G16B16A16Uint => 8,
            R16G16B16A16Snorm => 8,
            R16G16B16A16Sint => 8,
            R32G32Typeless => 8,
            R32G32Float => 8,
            R32G32Uint => 8,
            R32G32Sint => 8,
            R32G8X24Typeless => 8,
            D32FloatS8X24Uint => 8,
            R32FloatX8X24Typeless => 8,
            X32TypelessG8X24Uint => 8,
            R10G10B10A2Typeless => 4,
            R10G10B10A2Unorm => 4,
            R10G10B10A2Uint => 4,
            R11G11B10Float => 4,
            R8G8B8A8Typeless => 4,
            R8G8B8A8Unorm => 4,
            R8G8B8A8UnormSrgb => 4,
            R8G8B8A8Uint => 4,
            R8G8B8A8Snorm => 4,
            R8G8B8A8Sint => 4,
            R16G16Typeless => 4,
            R16G16Float => 4,
            R16G16Unorm => 4,
            R16G16Uint => 4,
            R16G16Snorm => 4,
            R16G16Sint => 4,
            R32Typeless => 4,
            D32Float => 4,
            R32Float => 4,
            R32Uint => 4,
            R32Sint => 4,
            R24G8Typeless => 4,
            D24UnormS8Uint => 4,
            R24UnormX8Typeless => 4,
            X24TypelessG8Uint => 4,
            R8G8Typeless => 2,
            R8G8Unorm => 2,
            R8G8Uint => 2,
            R8G8Snorm => 2,
            R8G8Sint => 2,
            R16Typeless => 2,
            R16Float => 2,
            D16Unorm => 2,
            R16Unorm => 2,
            R16Uint => 2,
            R16Snorm => 2,
            R16Sint => 2,
            R8Typeless => 1,
            R8Unorm => 1,
            R8Uint => 1,
            R8Snorm => 1,
            R8Sint => 1,
            A8Unorm => 1,
            R1Unorm => 0,
            R9G9B9E5SharedExponent => 4,
            R8G8_B8G8Unorm => 4,
            G8R8_G8B8Unorm => 4,
            Bc1Typeless => 0,
            Bc1Unorm => 0,
            Bc1UnormSrgb => 0,
            Bc2Typeless => 0,
            Bc2Unorm => 0,
            Bc2UnormSrgb => 0,
            Bc3Typeless => 0,
            Bc3Unorm => 0,
            Bc3UnormSrgb => 0,
            Bc4Typeless => 0,
            Bc4Unorm => 0,
            Bc4Snorm => 0,
            Bc5Typeless => 0,
            Bc5Unorm => 0,
            Bc5Snorm => 0,
            B5G6R5Unorm => 2,
            B5G5R5A1Unorm => 2,
            B8G8R8A8Unorm => 4,
            B8G8R8X8Unorm => 4,
            R10G10B10XrBiasA2Unorm => 4,
            B8G8R8A8Typeless => 4,
            B8G8R8A8UnormSrgb => 4,
            B8G8R8X8Typeless => 4,
            B8G8R8X8UnormSrgb => 4,
            Bc6hTypeless => 0,
            Bc6hUf16 => 0,
            Bc6hSf16 => 0,
            Bc7Typeless => 0,
            Bc7Unorm => 0,
            Bc7UnormSrgb => 0,
            Ayuv => 0,
            Y410 => 0,
            Y416 => 0,
            Nv12 => 0,
            P010 => 0,
            P016 => 0,
            Yuv420Opaque => 0,
            Yuy2 => 0,
            Y210 => 0,
            Y216 => 0,
            Nv11 => 0,
            Ai44 => 0,
            Ia44 => 0,
            P8 => 0,
            A8P8 => 0,
            B4G4R4A4Unorm => 2,
            P208 => 0,
            V208 => 0,
            V408 => 0,
        }
    }
}
