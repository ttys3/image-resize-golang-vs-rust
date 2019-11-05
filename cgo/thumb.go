package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"path/filepath"
	"time"

	"github.com/gographics/gmagick"
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

	thumbDir := "/tmp/thumb-cgo"
	os.RemoveAll(thumbDir)
	os.MkdirAll(thumbDir, os.ModePerm)
	startAll := time.Now()
	for i, f := range files {
		bname := filepath.Base(f)
		fmt.Printf("processing %d: %s\n", i, bname)
		start := time.Now()
		err := MkImageThumb(f, fmt.Sprintf("%s/%s", thumbDir, bname), size)
		if err != nil {
			fmt.Print(err)
		}
		fmt.Printf("%s took %v\n", bname, time.Since(start))
	}
	fmt.Printf("%s took %v\n", "Total", time.Since(startAll))
}

// import "github.com/gographics/gmagick"
func MkImageThumb(orig string, dest string, dstW int) error {
	if dstW <= 0 {
		panic("dst image width can not be 0")
	}
	gmagick.Initialize()
	defer gmagick.Terminate()

	mw := gmagick.NewMagickWand()
	defer mw.Destroy()

	if blob, err := ioutil.ReadFile(orig); err != nil {
		return err
	} else if err := mw.ReadImageBlob(blob); err != nil {
		return err
	}

	filter := gmagick.FILTER_BOX
	srcW := mw.GetImageWidth()
	srcH := mw.GetImageHeight()

	if int(srcW) <= dstW {
		return nil
	}

	// If new height is 0 then preserve aspect ratio, minimum 1px.
	tmpH := float64(dstW) * float64(srcH) / float64(srcW)
	dstH := int(math.Max(1.0, math.Floor(tmpH+0.5)))

	w := uint(dstW)
	h := uint(dstH)
	if err := mw.ResizeImage(w, h, filter, 1); err != nil {
		return err
	}
	if destFile, err := os.Create(dest); err != nil {
		return err
	} else {
		err := mw.WriteImageFile(destFile)
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


