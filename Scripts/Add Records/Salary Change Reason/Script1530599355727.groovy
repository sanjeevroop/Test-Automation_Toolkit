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

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Screens/Salary Change Reason/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/Salary Change Reason/input_Password'), '1234')

WebUI.doubleClick(findTestObject('Screens/Salary Change Reason/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/Salary Change Reason/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Salary Change Reason/button_Your Setup'))

WebUI.click(findTestObject('Screens/Salary Change Reason/a_Your Pay  Benefits'))

WebUI.click(findTestObject('Screens/Salary Change Reason/a_Lookups'))

WebUI.click(findTestObject('Screens/Salary Change Reason/a_Salary Change Reason'))

for (def row = 1; row <= findTestData('Minimumrates').getRowNumbers(); row++) {

WebUI.click(findTestObject('Screens/Salary Change Reason/a_Add Salary Change Reason'))

WebUI.setText(findTestObject('Screens/Salary Change Reason/input_SalaryChangeReason0'),findTestData('Salary Change Reason').getValue(
        1, row))

//WebUI.click(findTestObject('Screens/Salary Change Reason/input_Archive'))

WebUI.check(findTestObject('Screens/Salary Change Reason/input_Archive'))

WebUI.setText(findTestObject('Screens/Salary Change Reason/input_ReferenceCode0'),findTestData('Salary Change Reason').getValue(
        2, row))

WebUI.click(findTestObject('Screens/Salary Change Reason/button_Save'))

}

//WebUI.closeBrowser()

