import 'dart:math';
import 'dart:math' as math;

import 'package:flutter/material.dart';

enum Dimension { width, height, depth }

/// Like [Size], but in 3D.
class Dimensions {
  Dimensions(this.width, this.height, this.depth);
  Dimensions.all(double size) : this(size, size, size);
  static final zero = Dimensions.all(0);

  final double width;
  final double height;
  final double depth;

  double operator [](Dimension dimension) {
    switch (dimension) {
      case Dimension.width:
        return width;
      case Dimension.height:
        return height;
      case Dimension.depth:
        return depth;
    }
  }

  Dimensions operator +(Dimensions other) => Dimensions(
      width + other.width, height + other.height, depth + other.depth);
  Dimensions operator -(Dimensions other) => Dimensions(
      width - other.width, height - other.height, depth - other.depth);
  Dimensions operator *(double factor) =>
      Dimensions(width * factor, height * factor, depth * factor);

  bool operator ==(Object other) =>
      other is Dimensions &&
      width == other.width &&
      height == other.height &&
      depth == other.depth;
  int get hashCode => hashValues(width, height, depth);

  Dimensions copyWith({double? width, double? height, double? depth}) {
    return Dimensions(
        width ?? this.width, height ?? this.height, depth ?? this.depth);
  }

  Dimensions copyWithChangedDimension(Dimension dimension, double newSize) {
    switch (dimension) {
      case Dimension.width:
        return copyWith(width: newSize);
      case Dimension.height:
        return copyWith(height: newSize);
      case Dimension.depth:
        return copyWith(depth: newSize);
    }
  }
}

class Logo extends StatefulWidget {
  @override
  _LogoState createState() => _LogoState();
}

class _LogoState extends State<Logo> {
  var _dimensions = Dimensions.all(50);
  var _lastMutatedDimension = Dimension.height;

  @override
  void initState() {
    super.initState();
    Future.doWhile(() async {
      await Future.delayed(Duration(milliseconds: 1500));
      if (!this.mounted) return false;
      setState(_mutateDimensions);
      return true;
    });
  }

  void _mutateDimensions() {
    const min = 10.0;
    const max = 100.0;
    const minDelta = 20.0;

    final random = Random();

    double mutateRandomly(double size) {
      // Just choosing a random value doesn't look good, because it may be very
      // similar to the current one. The value should change at least by
      // `minDelta`.
      //
      // This leaves two ranges to choose the value from:
      //
      //    min                     current               max
      // xxxx|--------------------|xxxx|xxxx|--------------|xxxxxxxxxxx
      //      ^^^^^^^^^^^^^^^^^^^^           ^^^^^^^^^^^^^^
      final leftRangeMin = min;
      final leftRangeMax = math.max(min, size - minDelta);
      final leftRangeSize = leftRangeMax - leftRangeMin;
      final rightRangeMin = math.min(max, size + minDelta);
      final rightRangeMax = max;
      final rightRangeSize = rightRangeMax - rightRangeMin;

      final r = random.nextDouble() * (leftRangeSize + rightRangeSize);
      return r < leftRangeSize
          ? (leftRangeMin + r)
          : (rightRangeMin + r - leftRangeSize);
    }

    final dimension = Dimension.values
        .toSet()
        .difference({_lastMutatedDimension}).toList()[random.nextInt(2)];
    _dimensions = _dimensions.copyWithChangedDimension(
        dimension, mutateRandomly(_dimensions[dimension]));
    _lastMutatedDimension = dimension;
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 180,
      height: 200,
      child: TweenAnimationBuilder<Dimensions>(
        tween: Tween(begin: Dimensions.all(50), end: _dimensions),
        duration: Duration(seconds: 1),
        curve: Curves.easeInOutCubic,
        builder: (_, dimensions, __) {
          return CustomPaint(painter: _LogoPainter(dimensions: dimensions));
        },
      ),
    );
  }
}

class _LogoPainter extends CustomPainter {
  _LogoPainter({required this.dimensions});

  final Dimensions dimensions;

  @override
  void paint(Canvas canvas, Size size) {
    final frontHeight = dimensions.height;
    final rightWidth = 0.9 * dimensions.depth;
    final rightSlant = 0.5 * dimensions.depth;
    final leftWidth = 0.9 * dimensions.width;
    final leftSlant = 0.5 * dimensions.width;
    final focus = Offset(size.width / 2, size.height - frontHeight);

    // Left side.
    canvas.drawPath(
      Path()
        ..moveTo(focus.dx, focus.dy)
        ..lineTo(focus.dx - leftWidth, focus.dy - leftSlant)
        ..lineTo(focus.dx - leftWidth, focus.dy + frontHeight - leftSlant)
        ..lineTo(focus.dx, focus.dy + frontHeight)
        ..close(),
      Paint()..color = Color(0xff6a1cd5),
    );
    // Right side.
    canvas.drawPath(
      Path()
        ..moveTo(focus.dx, focus.dy)
        ..lineTo(focus.dx + rightWidth, focus.dy - rightSlant)
        ..lineTo(focus.dx + rightWidth, focus.dy + frontHeight - rightSlant)
        ..lineTo(focus.dx, focus.dy + frontHeight)
        ..close(),
      Paint()..color = Color(0xffef0164),
    );
    // Top side.
    canvas.drawPath(
      Path()
        ..moveTo(focus.dx, focus.dy)
        ..lineTo(focus.dx + rightWidth, focus.dy - rightSlant)
        ..lineTo(focus.dx + rightWidth - leftWidth,
            focus.dy - rightSlant - leftSlant)
        ..lineTo(focus.dx - leftWidth, focus.dy - leftSlant)
        ..close(),
      Paint()..color = Color(0xfffdc61e),
    );
  }

  @override
  bool shouldRepaint(_LogoPainter old) => dimensions != old.dimensions;
}
