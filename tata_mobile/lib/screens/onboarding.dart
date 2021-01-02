import 'package:flutter/material.dart';
import 'package:tata_mobile/screens/onboarding/primary_key.dart';
import 'package:tata_mobile/screens/onboarding/welcome.dart';

class Onboarding extends StatefulWidget {
  Onboarding({Key key}) : super(key: key);

  _OnboardingState createState() => _OnboardingState();
}

class _OnboardingState extends State<Onboarding> {
  PageController _pageController;

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return PageView(controller: _pageController, children: [
      Welcome(),
      PrimaryKey(),
    ]);
  }
}
