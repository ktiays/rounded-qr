//
//  Created by ktiays on 2024/10/21.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import Foundation
import QuartzCore
import RoundedQR

public struct RoundedQrCodeBuilder {

    /// The error correction level in a QR Code symbol.
    public enum ErrorCorrectionLevel: Int32 {
        /// The QR Code can tolerate about  7% erroneous codewords.
        case low = 0
        /// The QR Code can tolerate about 15% erroneous codewords.
        case medium = 1
        /// The QR Code can tolerate about 25% erroneous codewords.
        case quartile = 2
        /// The QR Code can tolerate about 30% erroneous codewords.
        case high = 3
    }

    public let data: Data
    var ecc: ErrorCorrectionLevel = .low
    var size: CGSize = .zero

    public init(_ data: Data) {
        self.data = data
    }

    public func ecc(_ level: ErrorCorrectionLevel) -> Self {
        var new = self
        new.ecc = level
        return new
    }

    public func size(_ size: CGSize) -> Self {
        var new = self
        new.size = size
        return new
    }

    public func build() -> CGPath {
        data.withUnsafeBytes { pointer in
            let path = CGMutablePath()
            guard
                let bytes = pointer.assumingMemoryBound(to: CChar.self)
                    .baseAddress
            else {
                return path
            }
            let length = pointer.count
            let builder = rqr_builder_create_with_data(
                bytes,
                length,
                ecc.rawValue,
                Float(size.width),
                Float(size.height)
            )
            let context = Unmanaged.passUnretained(path).toOpaque()
            rqr_builder_build_path(builder, context) { ctx, width, height in
                let path = Unmanaged<CGMutablePath>.fromOpaque(ctx!)
                    .takeUnretainedValue()
                path.move(to: .init(x: CGFloat(width), y: CGFloat(height)))
            } _: { ctx, width, height in
                let path = Unmanaged<CGMutablePath>.fromOpaque(ctx!)
                    .takeUnretainedValue()
                path.addLine(
                    to: .init(x: CGFloat(width), y: CGFloat(height))
                )
            } _: {
                ctx,
                width,
                height,
                radius,
                startAngle,
                endAngle,
                clockwise in
                let path = Unmanaged<CGMutablePath>.fromOpaque(ctx!)
                    .takeUnretainedValue()
                let isClockwise = clockwise != 1

                path.addArc(
                    center: .init(x: CGFloat(width), y: CGFloat(height)),
                    radius: CGFloat(radius),
                    startAngle: CGFloat(startAngle),
                    endAngle: CGFloat(endAngle),
                    clockwise: isClockwise
                )
            } _: { ctx in
                let path = Unmanaged<CGMutablePath>.fromOpaque(ctx!)
                    .takeUnretainedValue()
                path.closeSubpath()
            }
            return path
        }
    }
}
