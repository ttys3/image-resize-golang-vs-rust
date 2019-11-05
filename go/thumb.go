package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/disintegration/imaging"
)

var srcDir string
var size int

func init() {
	flag.StringVar(&srcDir, "src", "", "src dir")
	flag.IntVar(&size, "size", 128, "image with or height")
	flag.Parse()
}

func main() {
	files, err := filepath.Glob(fmt.Sprintf("%s/*.jpg", srcDir))
	if err != nil {
		fmt.Print(err)
		return
	}

	thumbDir := "/tmp/thumb-go"
	os.RemoveAll(thumbDir)
	os.MkdirAll(thumbDir, os.ModePerm)
	startAll := time.Now()
	for i, f := range files {
		bname := filepath.Base(f)
		fmt.Printf("processing %d: %s\n", i, bname)
		start := time.Now()
		err := MkImageThumb(f, fmt.Sprintf("%s/%s", thumbDir, bname), size, 0, 90)
		if err != nil {
			fmt.Print(err)
		}
		fmt.Printf("%s took %v\n", bname, time.Since(start))
	}
	fmt.Printf("%s took %v\n", "Total", time.Since(startAll))
}

func MkImageThumb(orig string, dest string, dstW int, dstH int, imgQuality int) error {
	// Open a test image.
	if src, err := imaging.Open(orig, imaging.AutoOrientation(false)); err != nil {
		return fmt.Errorf("failed to open image: %s, err: %v", orig, err)
	} else {
		// Resize the cropped image to width = dstW px preserving the aspect ratio.
		src = imaging.Resize(src, dstW, dstH, imaging.Box)
		// Save the resulting image as JPEG.
		err = imaging.Save(src, dest, imaging.JPEGQuality(imgQuality))
		if err != nil {
			return fmt.Errorf("failed to save image: %v", err)
		}
		return err
	}
}

// https://stackoverflow.com/a/45766707
// https://play.golang.org/p/8sKNmOdoEM
func elapsed(what string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v", what, time.Since(start))
	}
}


