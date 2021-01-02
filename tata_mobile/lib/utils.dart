import 'package:flutter/material.dart';

Size displaySize(BuildContext context) {
  return MediaQuery.of(context).size;
}

double displayHeight(BuildContext context) {
  return displaySize(context).height -
      MediaQuery.of(context).padding.top -
      kToolbarHeight;
}

double displayWidth(BuildContext context) {
  return displaySize(context).width;
}
