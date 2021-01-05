import 'package:flutter/material.dart';

class ScreenWave extends StatelessWidget {
  final double width;
  final double height;
  final double waveHeight;
  final Color color;

  ScreenWave(
      {Key key,
      @required this.width,
      @required this.height,
      @required this.waveHeight,
      @required this.color});

  @override
  Widget build(BuildContext context) {
    return CustomPaint(
        painter: WavePainter(color: color, waveHeight: waveHeight),
        size: Size(width, height + waveHeight));
  }
}

class WavePainter extends CustomPainter {
  final double waveHeight;
  final Color color;

  WavePainter({this.waveHeight, this.color});

  @override
  void paint(Canvas canvas, Size size) {
    final height = size.height - waveHeight;
    final width = size.width;
    final path = Path()
      ..moveTo(0, 0)
      ..lineTo(0, height)
      ..quadraticBezierTo(width / 4, height - waveHeight, width / 2, height)
      ..quadraticBezierTo(width * 3 / 4, height + waveHeight, width, height)
      ..lineTo(width, 0)
      ..lineTo(0, 0);
    final paint = Paint();
    paint.color = color;
    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(covariant WavePainter oldDelegate) {
    return (waveHeight != oldDelegate.waveHeight) ||
        (color != oldDelegate.color);
  }
}

// class ScreenWave extends StatelessWidget {
//   final double height;
//   final double width;
//   final double waveHeight;

//   ScreenWave(
//       {Key key,
//       @required this.width,
//       @required this.height,
//       @required this.waveHeight})
//       : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     // TODO: implement build
//     throw UnimplementedError();
//   }

//   Path getPath() {
//     final path = Path();
//     path.moveTo(0, 0);
//     path.lineTo(0, height);
//     path.quadraticBezierTo(width / 4, height - waveHeight, width / 2, height);
//     path.quadraticBezierTo(width * 3 / 4, height + waveHeight, width, height);
//     path.lineTo(width, 0);
//     path.lineTo(0, 0);
//     return path;
//   }
// }

// @override
// Path getClip(Size size) {
//   Path path = Path();
//   final double artboardW = 375+ (39) * progress;
//   final double artboardH = 177+ (124.69) * progress;
//   final double _xScaling = size.width / artboardW;
//   final double _yScaling = size.height / artboardH;
//   path.lineTo((193+ (-193) * progress) * _xScaling,(152.5+ (65.34100000000001) * progress) * _yScaling);
//   path.cubicTo((105.5+ (-105.5) * progress) * _xScaling,(95.5+ (122.34100000000001) * progress) * _yScaling,(0+ (19.144) * progress) * _xScaling,(152.5+ (113.41999999999996) * progress) * _yScaling,(0+ (67.237) * progress) * _xScaling,(152.5+ (113.41999999999996) * progress) * _yScaling,);
//   path.cubicTo((0+ (115.32999999999998) * progress) * _xScaling,(152.5+ (113.41999999999996) * progress) * _yScaling,(0+ (112.756) * progress) * _xScaling,(0+ (234.611) * progress) * _yScaling,(0+ (173.837) * progress) * _xScaling,(0+ (241.635) * progress) * _yScaling,);
//   path.cubicTo((0+ (234.91799999999998) * progress) * _xScaling,(0+ (248.659) * progress) * _yScaling,(375.5+ (-102.63) * progress) * _xScaling,(0+ (301.691) * progress) * _yScaling,(375.5+ (-46.888000000000034) * progress) * _xScaling,(0+ (301.691) * progress) * _yScaling,);
//   path.cubicTo((375.5+ (8.853999999999928) * progress) * _xScaling,(0+ (301.691) * progress) * _yScaling,(375.5+ (38.5) * progress) * _xScaling,(152.5+ (49.477000000000004) * progress) * _yScaling,(375.5+ (38.5) * progress) * _xScaling,(152.5+ (49.477000000000004) * progress) * _yScaling,);
//   path.cubicTo((375.5+ (38.5) * progress) * _xScaling,(152.5+ (49.477000000000004) * progress) * _yScaling,(280.5+ (133.5) * progress) * _xScaling,(209.5+ (-209.5) * progress) * _yScaling,(193+ (221) * progress) * _xScaling,(152.5+ (-152.5) * progress) * _yScaling,);
//   path.cubicTo((193+ (221) * progress) * _xScaling,(152.5+ (-152.5) * progress) * _yScaling,(193+ (-192.99999999999997) * progress) * _xScaling,(152.5+ (-152.5) * progress) * _yScaling,(193+ (-192.99999999999997) * progress) * _xScaling,(152.5+ (-152.5) * progress) * _yScaling,);
//   return path;
// }
