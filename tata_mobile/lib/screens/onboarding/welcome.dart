import 'package:flutter/material.dart';
import 'package:tata_mobile/screens/onboarding/screen_wave.dart';
import 'package:tata_mobile/utils.dart';

class Welcome extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => WelcomeState();
}

class WelcomeState extends State<Welcome> with SingleTickerProviderStateMixin {
  AnimationController _baseAnimation;
  Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _baseAnimation =
        AnimationController(vsync: this, duration: Duration(milliseconds: 500));
    _baseAnimation.forward();
    _animation = CurvedAnimation(
      parent: _baseAnimation,
      curve: Curves.easeOut,
      reverseCurve: Curves.easeIn,
    );
  }

  @override
  void dispose() {
    _baseAnimation.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final width = displayWidth(context).toDouble();
    final height = displayHeight(context).toDouble();
    return Stack(
      children: [
        AnimatedBuilder(
            animation: _animation,
            builder: (context, child) {
              final double progress = _animation.value;
              final waveHeight = 40.0 * progress;
              return ScreenWave(
                  width: width,
                  height: height / 4,
                  waveHeight: waveHeight,
                  color: Theme.of(context).primaryColor);
            })
      ],
    );
  }
}
