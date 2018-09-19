import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//WebUI.openBrowser('https://implementationnp.sdworx.co.uk/')

//WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/')

//WebUI.setText(findTestObject('Add Records/Add Records - Department Details/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Add Records/Add Records - Department Details/input_idSIButton9'))

//WebUI.delay(3)

//WebUI.mouseOver(findTestObject('Add Records/Add Records - Department Details/button_Your Setup'))

//WebUI.delay(3)

//WebUI.click(findTestObject('Add Records/Add Records - Department Details/button_Your Setup'))

//WebUI.delay(3)

//WebUI.mouseOver(findTestObject('Add Records/Add Records - Department Details/a_Your Payroll'))

//WebUI.delay(3)

//WebUI.click(findTestObject('Add Records/Add Records - Department Details/a_Departments'))



//WebUI.click(findTestObject('Add Records/Add Records - Department Details/a_Add Department'))

WebUI.delay(3)

for (def row = 1; row <= findTestData('Add Records - Department Details').getRowNumbers(); row++) {
	
WebUI.delay(5)
	
WebUI.click(findTestObject('Add Records/Add Records - Department Details/a_Add Department'))

WebUI.setText(findTestObject('Add Records/Add Records - Department Details/input_DepartmentNumber0'), findTestData
	('Add Records - Department Details').getValue(1, row))

WebUI.delay(1)

WebUI.setText(findTestObject('Add Records/Add Records - Department Details/input_DepartmentDescription0'), findTestData
	('Add Records - Department Details').getValue(2, row))

WebUI.delay(1)

WebUI.setText(findTestObject('Add Records/Add Records - Department Details/input_ReferenceCode0'), findTestData
	('Add Records - Department Details').getValue(3, row))


WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Add Records/Add Records - Department Details/button_Save'))

}
WebUI.delay(1)

WebUI.takeScreenshot('Screenshot/Department Details Screen/DepartmentDetails9.png')