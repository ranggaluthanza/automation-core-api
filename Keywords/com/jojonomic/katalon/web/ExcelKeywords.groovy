package com.jojonomic.katalon.web

import java.text.DateFormat
import java.text.SimpleDateFormat

import org.apache.poi.ss.usermodel.*
import org.apache.poi.ss.util.CellReference
import org.apache.xmlbeans.impl.values.XmlValueDisconnectedException

import com.kms.katalon.core.annotation.Keyword

public class ExcelKeywords {

	@Keyword
	static def getCellValue(Cell cell){
		switch (cell.getCellType()) {
			case Cell.CELL_TYPE_NUMERIC:
				try{
					if(DateUtil.isCellDateFormatted(cell)){
						DateFormat sdf = new SimpleDateFormat("MM/dd/yyyy")
						return sdf.format(cell.getDateCellValue())
					}
					else{
						if (cell.getNumericCellValue().toString().split("\\.").size ()>0){
							if (cell.getNumericCellValue().toString().split("\\.")[1] == "0"){
								return Integer.parseInt(cell.getNumericCellValue().toString().replace(".0",""))
							}
							else
								return cell.getNumericCellValue()
						}
						else {
							return cell.getNumericCellValue()
						}
					}
				}
				catch (XmlValueDisconnectedException ignored) {
					if (cell.getNumericCellValue().toString().split("\\.").size ()>0){
						if (cell.getNumericCellValue().toString().split("\\.")[1] == "0"){
							return Integer.parseInt(cell.getNumericCellValue().toString().replace(".0",""))
						}
						else {
							return cell.getNumericCellValue()
						}
					}
					else {
						return cell.getNumericCellValue()
					}
				}

				break
			case Cell.CELL_TYPE_STRING:
				return cell.getStringCellValue()
				break
			case Cell.CELL_TYPE_BLANK:
				return ""
				break
			case Cell.CELL_TYPE_BOOLEAN:
				return cell.getBooleanCellValue()
				break
			case Cell.CELL_TYPE_FORMULA:
				switch(cell.getCachedFormulaResultType()) {
					case Cell.CELL_TYPE_NUMERIC:
					if (cell.getNumericCellValue().toString().split("\\.").size ()>0){
						if (cell.getNumericCellValue().toString().split("\\.")[1] == "0"){
							return Integer.parseInt(cell.getNumericCellValue().toString().replace(".0",""))
						}
						else
							return cell.getNumericCellValue()
					}
					else {
						return cell.getNumericCellValue()
					}
					case Cell.CELL_TYPE_STRING:
					return cell.getStringCellValue()
					break
				}
		}
	}

	@Keyword
	static Sheet getExcelSheet(String filePath, int sheetIndex = 0) throws Exception {
		FileInputStream fs = new FileInputStream(filePath)
		String[] arrPath = filePath.split("\\.")
		String fileExt = arrPath[arrPath.length - 1]
		Workbook workbook = WorkbookFactory.create(new File(filePath))
		return workbook.getSheetAt(sheetIndex)
	}

	@Keyword
	static Cell getCellByIndex(Sheet sheet, int rowIdx, int colIdx){
		Row row = sheet.getRow(rowIdx)
		Cell cell = row.getCell(colIdx)
		return cell
	}

	@Keyword
	static def getCellValueByIndex(Sheet sheet, int rowIdx, int colIdx){
		Cell cell = getCellByIndex(sheet, rowIdx, colIdx)
		if (null !=cell) {
			return getCellValue(cell)
		}
		else return ""
	}

	@Keyword
	static Cell getCellByAddress(Sheet sheet, String cellAddress){
		CellReference cr = new CellReference(cellAddress)
		Row row = sheet.getRow(cr.getRow())
		Cell cell = row.getCell(cr.getCol())
		return cell
	}

	@Keyword
	static def getCellValueByAddress(Sheet sheet, String cellAddress){
		Cell cell = getCellByAddress(sheet, cellAddress)
		if (null !=cell) {
			return getCellValue(cell)
		}
		else return ""
	}
}
