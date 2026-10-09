
## 7. Centring across viewport and zoom (2026-10-09, update)

Reported: the radio looked centred at first, then off-centre after resizing.

- **Viewport width:** the ring is measured at widths from 320 to 1280px. The layout
  offset is 0 at every width, so resizing the window does not move the layout.
- **Browser zoom:** the first version drew the ring and dot as boxes. Their edges
  rounded to device pixels, so the drawn centre could shift by up to one device pixel
  at 80% and 90% zoom.
- **Fix:** the ring and dot are now SVG circles on a 20-unit grid, and the focus ring
  sits on the 40dp state layer.
- **After the fix:** the ring's ink centre is within 0.8 device pixels of the expected
  centre at 67%, 80%, 90%, 100%, 110%, 125% and 150% zoom. Measurement uses integer
  pixel bounds, which resolve about 0.5 pixel, so most of the residual is measurement
  noise. Raw data: [`radio-zoom-samples.json`](radio-zoom-samples.json).
- **Not verified:** Safari and Firefox, and the visual result on the user's device.
