import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

enum Picture { P2P, Fees, Launch }

class OnboardingScreen extends StatelessWidget {
  final String _header;
  final String _body;
  final Picture _picture;
  final int _number;

  OnboardingScreen(
      {@required header, @required body, @required picture, @required number})
      : _header = header,
        _body = body,
        _picture = picture,
        _number = number;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        SvgPicture.asset(this.pathToPicture(), semanticsLabel: 'Visual'),
        Text(this._header),
        Text(this._body),
      ],
    );
  }

  String pathToPicture() {
    switch (this._picture) {
      case Picture.P2P:
        return 'assets/svg/phone_p2p.svg';
      case Picture.Fees:
        return 'assets/svg/phone_fees.svg';
      case Picture.Launch:
        return 'assets/svg/phone_launch.svg';
    }
    return '';
  }
}
