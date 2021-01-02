import 'package:flutter/material.dart';
import 'package:tata_mobile/utils.dart';
import 'dart:math';

class Welcome extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
        color: Theme.of(context).backgroundColor,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Row(
              children: [
                PhoneIcon(),
                PhoneIcon(
                  inverse: true,
                )
              ],
            ),
            Text("Welcome")
          ],
        ));
  }
}

class PhoneIcon extends StatelessWidget {
  final bool inverse;
  PhoneIcon({this.inverse: false});

  @override
  Widget build(BuildContext context) {
    return Transform(
        alignment: Alignment.center,
        transform: this.inverse ? Matrix4.rotationY(pi) : Matrix4.identity(),
        child: Icon(
          Icons.phonelink_ring,
          color: Theme.of(context).primaryColor,
          size: displayWidth(context) / 4,
        ));
  }
}
